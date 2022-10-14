use {
    crate::errors::ClockworkError,
    anchor_lang::{
        prelude::*,
        solana_program::{
            instruction::Instruction,
            program::{get_return_data, invoke_signed},
        },
        AnchorDeserialize, AnchorSerialize,
    },
    chrono::{DateTime, NaiveDateTime, Utc},
    clockwork_cron::Schedule,
    clockwork_network_program::objects::{Fee, Penalty, Pool, Worker},
    clockwork_utils::*,
    std::{
        collections::hash_map::DefaultHasher,
        convert::TryFrom,
        hash::{Hash, Hasher},
        str::FromStr,
    },
};

pub const SEED_QUEUE: &[u8] = b"queue";

/// The default rate limit to initialize queues with
const DEFAULT_RATE_LIMIT: u64 = 10;

/// The maximum rate limit which may be set on queue.
const MAX_RATE_LIMIT: u64 = 32;

/// The Minimum crank fee that may be set on a queue.
const MINIMUM_FEE: u64 = 1000;

/// The Number of lamports to reimburse the worker with after they've submitted a transaction's worth of cranks.
const TRANSACTION_BASE_FEE_REIMBURSEMENT: u64 = 5_000;

/// Tracks the current state of a transaction thread on Solana.
#[account]
#[derive(Debug)]
pub struct Queue {
    /// The owner of this queue.
    pub authority: Pubkey,
    /// The cluster clock at the moment the queue was created.
    pub created_at: ClockData,
    /// The context of the current thread execution state.
    pub exec_context: Option<ExecContext>,
    /// The number of lamports to payout to workers per crank.
    pub fee: u64,
    /// The id of the queue, given by the authority.
    pub id: String,
    /// The instruction to kick-off the thread.
    pub kickoff_instruction: InstructionData,
    /// The next instruction in the thread.
    pub next_instruction: Option<InstructionData>,
    /// Whether or not the queue is currently paused.
    pub paused: bool,
    /// The maximum number of cranks allowed per slot.
    pub rate_limit: u64,
    /// The triggering event to kickoff a thread.
    pub trigger: Trigger,
}

impl Queue {
    /// Derive the pubkey of a queue account.
    pub fn pubkey(authority: Pubkey, id: String) -> Pubkey {
        Pubkey::find_program_address(&[SEED_QUEUE, authority.as_ref(), id.as_bytes()], &crate::ID).0
    }
}

impl TryFrom<Vec<u8>> for Queue {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Queue::try_deserialize(&mut data.as_slice())
    }
}

impl Hash for Queue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.authority.hash(state);
        self.id.hash(state);
    }
}

impl PartialEq for Queue {
    fn eq(&self, other: &Self) -> bool {
        self.authority.eq(&other.authority) && self.id.eq(&other.id)
    }
}

impl Eq for Queue {}

/// The properties of queues which are updatable.
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct QueueSettings {
    pub fee: Option<u64>,
    pub kickoff_instruction: Option<InstructionData>,
    pub rate_limit: Option<u64>,
    pub trigger: Option<Trigger>,
}

/// Trait for reading and writing to a queue account.
pub trait QueueAccount {
    /// Get the pubkey of the queue account.
    fn pubkey(&self) -> Pubkey;

    /// Initialize the account to hold queue object.
    fn init(
        &mut self,
        authority: Pubkey,
        id: String,
        kickoff_instruction: InstructionData,
        trigger: Trigger,
    ) -> Result<()>;

    /// Crank the queue. Call out to the target program and parse the response for a next instruction.
    fn crank(
        &mut self,
        account_infos: &[AccountInfo],
        bump: u8,
        fee: &mut Account<Fee>,
        penalty: &mut Account<Penalty>,
        pool: &Account<Pool>,
        signatory: &mut Signer,
        worker: &Account<Worker>,
    ) -> Result<()>;

    fn kickoff(&mut self, data_hash: Option<u64>, remaining_accounts: &[AccountInfo])
        -> Result<()>;

    /// Reallocate the memory allocation for the account.
    fn realloc(&mut self) -> Result<()>;

    fn update(&mut self, settings: QueueSettings) -> Result<()>;
}

impl QueueAccount for Account<'_, Queue> {
    fn pubkey(&self) -> Pubkey {
        Queue::pubkey(self.authority, self.id.clone())
    }

    fn init(
        &mut self,
        authority: Pubkey,
        id: String,
        kickoff_instruction: InstructionData,
        trigger: Trigger,
    ) -> Result<()> {
        self.authority = authority.key();
        self.created_at = Clock::get().unwrap().into();
        self.exec_context = None;
        self.fee = MINIMUM_FEE;
        self.id = id;
        self.kickoff_instruction = kickoff_instruction;
        self.next_instruction = None;
        self.paused = false;
        self.rate_limit = DEFAULT_RATE_LIMIT;
        self.trigger = trigger;
        Ok(())
    }

    fn crank(
        &mut self,
        account_infos: &[AccountInfo],
        bump: u8,
        fee: &mut Account<Fee>,
        penalty: &mut Account<Penalty>,
        pool: &Account<Pool>,
        signatory: &mut Signer,
        worker: &Account<Worker>,
    ) -> Result<()> {
        // Record the worker's lamports before invoking inner ixs
        let signatory_lamports_pre = signatory.lamports();

        // Get the instruction to crank
        let kickoff_instruction: &InstructionData = &self.clone().kickoff_instruction;
        let next_instruction: &Option<InstructionData> = &self.clone().next_instruction;
        let instruction = next_instruction.as_ref().unwrap_or(kickoff_instruction);

        // Inject the signatory's pubkey for the Clockwork payer ID
        let normalized_accounts: &mut Vec<AccountMeta> = &mut vec![];
        instruction.accounts.iter().for_each(|acc| {
            let acc_pubkey = if acc.pubkey == clockwork_utils::PAYER_PUBKEY {
                signatory.key()
            } else {
                acc.pubkey
            };
            normalized_accounts.push(AccountMeta {
                pubkey: acc_pubkey,
                is_signer: acc.is_signer,
                is_writable: acc.is_writable,
            });
        });

        // Invoke the provided instruction
        invoke_signed(
            &Instruction {
                program_id: instruction.program_id,
                data: instruction.data.clone(),
                accounts: normalized_accounts.to_vec(),
            },
            account_infos,
            &[&[
                SEED_QUEUE,
                self.authority.as_ref(),
                self.id.as_bytes(),
                &[bump],
            ]],
        )?;

        // Verify that the inner ix did not write data to the signatory address
        require!(signatory.data_is_empty(), ClockworkError::UnauthorizedWrite);

        // Parse the crank response
        match get_return_data() {
            None => {
                self.next_instruction = None;
            }
            Some((program_id, return_data)) => {
                require!(
                    program_id.eq(&instruction.program_id),
                    ClockworkError::InvalidCrankResponse
                );
                let crank_response = CrankResponse::try_from_slice(return_data.as_slice())
                    .map_err(|_err| ClockworkError::InvalidCrankResponse)?;

                // Update the queue with the crank response.
                if let Some(kickoff_instruction) = crank_response.kickoff_instruction {
                    self.kickoff_instruction = kickoff_instruction;
                }
                self.next_instruction = crank_response.next_instruction;
            }
        };

        // Increment the crank count
        let current_slot = Clock::get().unwrap().slot;
        match self.exec_context {
            None => return Err(ClockworkError::InvalidQueueState.into()),
            Some(exec_context) => {
                // Update the exec context
                self.exec_context = Some(ExecContext {
                    cranks_since_reimbursement: exec_context
                        .cranks_since_reimbursement
                        .checked_add(1)
                        .unwrap(),
                    cranks_since_slot: if current_slot == exec_context.last_crank_at {
                        exec_context.cranks_since_slot.checked_add(1).unwrap()
                    } else {
                        1
                    },
                    last_crank_at: current_slot,
                    trigger_context: exec_context.trigger_context,
                });
            }
        }

        // Realloc the queue account
        self.realloc()?;

        // Reimbursement signatory for lamports paid during inner ix
        let signatory_lamports_post = signatory.lamports();
        let signatory_reimbursement = signatory_lamports_pre
            .checked_sub(signatory_lamports_post)
            .unwrap();
        **self.to_account_info().try_borrow_mut_lamports()? = self
            .to_account_info()
            .lamports()
            .checked_sub(signatory_reimbursement)
            .unwrap();
        **signatory.to_account_info().try_borrow_mut_lamports()? = signatory
            .to_account_info()
            .lamports()
            .checked_add(signatory_reimbursement)
            .unwrap();

        // Debit the crank fee from the queue account.
        // If the worker is in the pool, pay fee to the worker's fee account.
        // Otherwise, pay fee to the worker's penalty account.
        **self.to_account_info().try_borrow_mut_lamports()? = self
            .to_account_info()
            .lamports()
            .checked_sub(self.fee)
            .unwrap();
        if pool.clone().into_inner().workers.contains(&worker.key()) {
            **fee.to_account_info().try_borrow_mut_lamports()? = fee
                .to_account_info()
                .lamports()
                .checked_add(self.fee)
                .unwrap();
        } else {
            **penalty.to_account_info().try_borrow_mut_lamports()? = penalty
                .to_account_info()
                .lamports()
                .checked_add(self.fee)
                .unwrap();
        }

        // If the self has no more work or the number of cranks since the last payout has reached the rate limit,
        // reimburse the worker for the transaction base fee.
        match self.exec_context {
            None => {
                return Err(ClockworkError::InvalidQueueState.into());
            }
            Some(exec_context) => {
                if self.next_instruction.is_none()
                    || exec_context.cranks_since_reimbursement >= self.rate_limit
                {
                    // Pay reimbursment for base transaction fee
                    **self.to_account_info().try_borrow_mut_lamports()? = self
                        .to_account_info()
                        .lamports()
                        .checked_sub(TRANSACTION_BASE_FEE_REIMBURSEMENT)
                        .unwrap();
                    **signatory.to_account_info().try_borrow_mut_lamports()? = signatory
                        .to_account_info()
                        .lamports()
                        .checked_add(TRANSACTION_BASE_FEE_REIMBURSEMENT)
                        .unwrap();

                    // Update the exec context to mark that a reimbursement happened this slot.
                    self.exec_context = Some(ExecContext {
                        cranks_since_reimbursement: 0,
                        ..exec_context
                    });
                }
            }
        }

        Ok(())
    }

    fn realloc(&mut self) -> Result<()> {
        // Realloc memory for the queue account
        let data_len = 8 + self.try_to_vec()?.len();
        self.to_account_info().realloc(data_len, false)?;
        Ok(())
    }

    fn update(&mut self, settings: QueueSettings) -> Result<()> {
        // If provided, update the queue's fee.
        if let Some(fee) = settings.fee {
            self.fee = fee;
        }

        // If provided, update the queue's first instruction
        if let Some(kickoff_instruction) = settings.kickoff_instruction {
            self.kickoff_instruction = kickoff_instruction;
        }

        // If provided, update the rate_limit
        if let Some(rate_limit) = settings.rate_limit {
            require!(
                rate_limit.le(&MAX_RATE_LIMIT),
                ClockworkError::MaxRateLimitExceeded
            );
            self.rate_limit = rate_limit;
        }

        // If provided, update the queue's trigger and reset the exec context
        if let Some(trigger) = settings.trigger {
            self.trigger = trigger;
            self.exec_context = None;
        }

        Ok(())
    }

    fn kickoff(
        &mut self,
        data_hash: Option<u64>,
        remaining_accounts: &[AccountInfo],
    ) -> Result<()> {
        let clock = Clock::get().unwrap();
        match self.trigger.clone() {
            Trigger::Account { pubkey } => {
                // Require the provided data hash is non-null.
                let data_hash = match data_hash {
                    None => return Err(ClockworkError::DataHashNotPresent.into()),
                    Some(data_hash) => data_hash,
                };

                // Verify proof that account data has been updated.
                match remaining_accounts.first() {
                    None => {}
                    Some(account_info) => {
                        // Verify the remaining account is the account this queue is listening for.
                        require!(
                            pubkey.eq(account_info.key),
                            ClockworkError::TriggerNotActive
                        );

                        // Begin computing the data hash of this account.
                        let mut hasher = DefaultHasher::new();
                        let data = &account_info.try_borrow_data().unwrap();
                        data.to_vec().hash(&mut hasher);

                        // Check the exec context for the prior data hash.
                        let expected_data_hash = match self.exec_context.clone() {
                            None => {
                                // This queue has not begun executing yet.
                                // There is no prior data hash to include in our hash.
                                hasher.finish()
                            }
                            Some(exec_context) => {
                                match exec_context.trigger_context {
                                    TriggerContext::Account {
                                        data_hash: prior_data_hash,
                                    } => {
                                        // Inject the prior data hash as a seed.
                                        prior_data_hash.hash(&mut hasher);
                                        hasher.finish()
                                    }
                                    _ => return Err(ClockworkError::InvalidQueueState.into()),
                                }
                            }
                        };

                        // Verify the data hash provided by the worker is equal to the expected data hash.
                        // This proves the account has been updated since the last crank and the worker has seen the new data.
                        require!(
                            data_hash.eq(&expected_data_hash),
                            ClockworkError::TriggerNotActive
                        );

                        // Set a new exec context with the new data hash and slot number.
                        self.exec_context = Some(ExecContext {
                            cranks_since_reimbursement: 0,
                            cranks_since_slot: 0,
                            last_crank_at: clock.slot,
                            trigger_context: TriggerContext::Account { data_hash },
                        })
                    }
                }
            }
            Trigger::Cron {
                schedule,
                skippable,
            } => {
                // Get the reference timestamp for calculating the queue's scheduled target timestamp.
                let reference_timestamp = match self.exec_context.clone() {
                    None => self.created_at.unix_timestamp,
                    Some(exec_context) => match exec_context.trigger_context {
                        TriggerContext::Cron { started_at } => started_at,
                        _ => return Err(ClockworkError::InvalidQueueState.into()),
                    },
                };

                // Verify the current timestamp is greater than or equal to the threshold timestamp.
                let threshold_timestamp = next_timestamp(reference_timestamp, schedule.clone())
                    .ok_or(ClockworkError::TriggerNotActive)?;
                require!(
                    clock.unix_timestamp >= threshold_timestamp,
                    ClockworkError::TriggerNotActive
                );

                // Set the trigger context started_at to be the threshold timestamp that had to be met.
                let mut started_at = threshold_timestamp;

                // If the schedule is marked as skippable and kickoff thresholds have been missed,
                // set the started_at of the trigger context to be the threshold moment just before the current timestamp.
                if skippable {
                    if let Some(next_threshold_timestamp) =
                        next_timestamp(threshold_timestamp, schedule.clone())
                    {
                        if clock.unix_timestamp.gt(&next_threshold_timestamp) {
                            started_at = prev_timestamp(clock.unix_timestamp, schedule)
                                .ok_or(ClockworkError::TriggerNotActive)?
                        }
                    }
                };

                // Set the exec context.
                self.exec_context = Some(ExecContext {
                    cranks_since_reimbursement: 0,
                    cranks_since_slot: 0,
                    last_crank_at: clock.slot,
                    trigger_context: TriggerContext::Cron { started_at },
                });
            }
            Trigger::Immediate => {
                // Set the exec context.
                require!(
                    self.exec_context.is_none(),
                    ClockworkError::InvalidQueueState
                );
                self.exec_context = Some(ExecContext {
                    cranks_since_reimbursement: 0,
                    cranks_since_slot: 0,
                    last_crank_at: clock.slot,
                    trigger_context: TriggerContext::Immediate,
                });
            }
        }

        // If we make it here, the trigger is active. Update the next instruction and be done.
        self.next_instruction = Some(self.kickoff_instruction.clone());

        // Realloc the queue account
        self.realloc()?;

        Ok(())
    }
}

/// The triggering conditions of a queue.
#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, PartialEq)]
pub enum Trigger {
    /// Allows a queue to subscribe to an accout and be cranked whenever the data of that account changes.
    Account {
        /// The address of the account to subscribe to.
        pubkey: Pubkey,
    },

    /// Allows a queue to be cranked according to a one-time or recurring schedule.
    Cron {
        /// The schedule in cron syntax. Value must be parsable by the `clockwork_cron` package.
        schedule: String,

        /// Boolean value indicating whether triggering moments may be skipped if they are missed (e.g. due to network downtime).
        /// If false, any "missed" triggering moments will simply be cranked as soon as the network comes back online.
        skippable: bool,
    },

    /// Allows a queue to be cranked as soon as it's created.
    Immediate,
}

/// The execution context of a particular transaction thread.
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ExecContext {
    /// Number of cranks since the last tx reimbursement.
    pub cranks_since_reimbursement: u64,

    /// Number of cranks in this slot.
    pub cranks_since_slot: u64,

    /// Slot of the last crank
    pub last_crank_at: u64,

    /// Context for the triggering condition
    pub trigger_context: TriggerContext,
}

/// The event which allowed a particular transaction thread to be triggered.
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum TriggerContext {
    /// A running hash of the observed account data.
    Account {
        /// The account's data hash.
        data_hash: u64,
    },

    /// A cron execution context.
    Cron {
        /// The threshold moment the schedule was waiting for.
        started_at: i64,
    },

    /// The immediate trigger context.
    Immediate,
}

fn next_timestamp(after: i64, schedule: String) -> Option<i64> {
    Schedule::from_str(&schedule)
        .unwrap()
        .next_after(&DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(after, 0),
            Utc,
        ))
        .take()
        .map(|datetime| datetime.timestamp())
}

fn prev_timestamp(before: i64, schedule: String) -> Option<i64> {
    Schedule::from_str(&schedule)
        .unwrap()
        .prev_before(&DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(before, 0),
            Utc,
        ))
        .take()
        .map(|datetime| datetime.timestamp())
}
