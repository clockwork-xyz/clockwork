use {
    crate::errors::ClockworkError,
    crate::state::*,
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
    clockwork_network_program::state::{Fee, Penalty, Pool, Worker},
    std::{
        collections::hash_map::DefaultHasher,
        convert::TryFrom,
        hash::{Hash, Hasher},
        str::FromStr,
    },
};

pub const SEED_THREAD: &[u8] = b"thread";

/// The default rate limit to initialize threads with
const DEFAULT_RATE_LIMIT: u64 = 10;

/// The maximum rate limit which may be set on thread.
const MAX_RATE_LIMIT: u64 = 32;

/// The minimum exec fee that may be set on a thread.
const MINIMUM_FEE: u64 = 1000;

/// The number of lamports to reimburse the worker with after they've submitted a transaction's worth of exec instructions.
const TRANSACTION_BASE_FEE_REIMBURSEMENT: u64 = 5_000;

/// Tracks the current state of a transaction thread on Solana.
#[account]
#[derive(Debug)]
pub struct Thread {
    /// The owner of this thread.
    pub authority: Pubkey,
    /// The cluster clock at the moment the thread was created.
    pub created_at: ClockData,
    /// The context of the thread's current execution state.
    pub exec_context: Option<ExecContext>,
    /// The number of lamports to payout to workers per execution.
    pub fee: u64,
    /// The id of the thread, given by the authority.
    pub id: String,
    /// The instruction to kick-off the thread.
    pub kickoff_instruction: InstructionData,
    /// The next instruction in the thread.
    pub next_instruction: Option<InstructionData>,
    /// Whether or not the thread is currently paused.
    pub paused: bool,
    /// The maximum number of execs allowed per slot.
    pub rate_limit: u64,
    /// The triggering event to kickoff a thread.
    pub trigger: Trigger,
}

impl Thread {
    /// Derive the pubkey of a thread account.
    pub fn pubkey(authority: Pubkey, id: String) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_THREAD, authority.as_ref(), id.as_bytes()],
            &crate::ID,
        )
        .0
    }
}

impl TryFrom<Vec<u8>> for Thread {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Thread::try_deserialize(&mut data.as_slice())
    }
}

impl Hash for Thread {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.authority.hash(state);
        self.id.hash(state);
    }
}

impl PartialEq for Thread {
    fn eq(&self, other: &Self) -> bool {
        self.authority.eq(&other.authority) && self.id.eq(&other.id)
    }
}

impl Eq for Thread {}

/// The properties of threads which are updatable.
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ThreadSettings {
    pub fee: Option<u64>,
    pub kickoff_instruction: Option<InstructionData>,
    pub rate_limit: Option<u64>,
    pub trigger: Option<Trigger>,
}

/// Trait for reading and writing to a thread account.
pub trait ThreadAccount {
    /// Get the pubkey of the thread account.
    fn pubkey(&self) -> Pubkey;

    /// Initialize the account to hold thread object.
    fn init(
        &mut self,
        authority: Pubkey,
        id: String,
        kickoff_instruction: InstructionData,
        trigger: Trigger,
    ) -> Result<()>;

    /// Execute the next instruction on the thread.
    fn exec(
        &mut self,
        account_infos: &[AccountInfo],
        bump: u8,
        fee: &mut Account<Fee>,
        penalty: &mut Account<Penalty>,
        pool: &Account<Pool>,
        signatory: &mut Signer,
        worker: &Account<Worker>,
    ) -> Result<()>;

    fn kickoff(&mut self, remaining_accounts: &[AccountInfo]) -> Result<()>;

    /// Reallocate the memory allocation for the account.
    fn realloc(&mut self) -> Result<()>;

    fn update(&mut self, settings: ThreadSettings) -> Result<()>;
}

impl ThreadAccount for Account<'_, Thread> {
    fn pubkey(&self) -> Pubkey {
        Thread::pubkey(self.authority, self.id.clone())
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

    fn exec(
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

        // Get the instruction to execute
        // TODO Just grab the next_instruction here. We have already verified that it is not null.
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
                SEED_THREAD,
                self.authority.as_ref(),
                self.id.as_bytes(),
                &[bump],
            ]],
        )?;

        // Verify that the inner ix did not write data to the signatory address
        require!(signatory.data_is_empty(), ClockworkError::UnauthorizedWrite);

        // Parse the exec response
        match get_return_data() {
            None => {
                self.next_instruction = None;
            }
            Some((program_id, return_data)) => {
                require!(
                    program_id.eq(&instruction.program_id),
                    ClockworkError::InvalidThreadResponse
                );
                let exec_response = ThreadResponse::try_from_slice(return_data.as_slice())
                    .map_err(|_err| ClockworkError::InvalidThreadResponse)?;

                // Update the thread with the exec response.
                if let Some(kickoff_instruction) = exec_response.kickoff_instruction {
                    self.kickoff_instruction = kickoff_instruction;
                }
                self.next_instruction = exec_response.next_instruction;
            }
        };

        // Increment the exec count
        let current_slot = Clock::get().unwrap().slot;
        match self.exec_context {
            None => return Err(ClockworkError::InvalidThreadState.into()),
            Some(exec_context) => {
                // Update the exec context
                self.exec_context = Some(ExecContext {
                    execs_since_reimbursement: exec_context
                        .execs_since_reimbursement
                        .checked_add(1)
                        .unwrap(),
                    execs_since_slot: if current_slot == exec_context.last_exec_at {
                        exec_context.execs_since_slot.checked_add(1).unwrap()
                    } else {
                        1
                    },
                    last_exec_at: current_slot,
                    ..exec_context
                });
            }
        }

        // Realloc the thread account
        self.realloc()?;

        // Reimbursement signatory for lamports paid during inner ix
        let signatory_lamports_post = signatory.lamports();
        let signatory_reimbursement =
            signatory_lamports_pre.saturating_sub(signatory_lamports_post);
        if signatory_reimbursement.gt(&0) {
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
        }

        // Debit the fee from the thread account.
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

        // If the self has no more work or the number of execs since the last payout has reached the rate limit,
        // reimburse the worker for the transaction base fee.
        match self.exec_context {
            None => {
                return Err(ClockworkError::InvalidThreadState.into());
            }
            Some(exec_context) => {
                if self.next_instruction.is_none()
                    || exec_context.execs_since_reimbursement >= self.rate_limit
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
                        execs_since_reimbursement: 0,
                        ..exec_context
                    });
                }
            }
        }

        Ok(())
    }

    fn realloc(&mut self) -> Result<()> {
        // Realloc memory for the thread account
        let data_len = 8 + self.try_to_vec()?.len();
        self.to_account_info().realloc(data_len, false)?;
        Ok(())
    }

    fn update(&mut self, settings: ThreadSettings) -> Result<()> {
        // If provided, update the thread's fee.
        if let Some(fee) = settings.fee {
            self.fee = fee;
        }

        // If provided, update the thread's first instruction
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

        // If provided, update the thread's trigger and reset the exec context
        if let Some(trigger) = settings.trigger {
            // Require the thread is not in the middle of processing.
            require!(self.next_instruction.is_none(), ClockworkError::ThreadBusy);
            self.trigger = trigger;
            self.exec_context = None;
        }

        Ok(())
    }

    fn kickoff(&mut self, remaining_accounts: &[AccountInfo]) -> Result<()> {
        let clock = Clock::get().unwrap();
        match self.trigger.clone() {
            Trigger::Account {
                address,
                offset,
                size,
            } => {
                // Verify proof that account data has been updated.
                match remaining_accounts.first() {
                    None => {}
                    Some(account_info) => {
                        // Verify the remaining account is the account this thread is listening for.
                        require!(
                            address.eq(account_info.key),
                            ClockworkError::TriggerNotActive
                        );

                        // Begin computing the data hash of this account.
                        let mut hasher = DefaultHasher::new();
                        let data = &account_info.try_borrow_data().unwrap();
                        let range_end = offset.checked_add(size).unwrap();
                        if data.len().gt(&range_end) {
                            data[offset..range_end].hash(&mut hasher);
                        } else {
                            data[offset..].hash(&mut hasher)
                        }
                        let data_hash = hasher.finish();

                        // Verify the data hash is different than the prior data hash.
                        if let Some(exec_context) = self.exec_context {
                            match exec_context.trigger_context {
                                TriggerContext::Account {
                                    data_hash: prior_data_hash,
                                } => {
                                    require!(
                                        data_hash.ne(&prior_data_hash),
                                        ClockworkError::TriggerNotActive
                                    )
                                }
                                _ => return Err(ClockworkError::InvalidThreadState.into()),
                            }
                        }

                        // Set a new exec context with the new data hash and slot number.
                        self.exec_context = Some(ExecContext {
                            execs_since_reimbursement: 0,
                            execs_since_slot: 0,
                            last_exec_at: clock.slot,
                            trigger_context: TriggerContext::Account { data_hash },
                        })
                    }
                }
            }
            Trigger::Cron {
                schedule,
                skippable,
            } => {
                // Get the reference timestamp for calculating the thread's scheduled target timestamp.
                let reference_timestamp = match self.exec_context.clone() {
                    None => self.created_at.unix_timestamp,
                    Some(exec_context) => match exec_context.trigger_context {
                        TriggerContext::Cron { started_at } => started_at,
                        _ => return Err(ClockworkError::InvalidThreadState.into()),
                    },
                };

                // Verify the current timestamp is greater than or equal to the threshold timestamp.
                let threshold_timestamp = next_timestamp(reference_timestamp, schedule.clone())
                    .ok_or(ClockworkError::TriggerNotActive)?;
                require!(
                    clock.unix_timestamp.ge(&threshold_timestamp),
                    ClockworkError::TriggerNotActive
                );

                // If the schedule is marked as skippable, set the started_at of the exec context to be the current timestamp.
                // Otherwise, the exec context must iterate through each scheduled kickoff moment.
                let started_at = if skippable {
                    clock.unix_timestamp
                } else {
                    threshold_timestamp
                };

                // Set the exec context.
                self.exec_context = Some(ExecContext {
                    execs_since_reimbursement: 0,
                    execs_since_slot: 0,
                    last_exec_at: clock.slot,
                    trigger_context: TriggerContext::Cron { started_at },
                });
            }
            Trigger::Immediate => {
                // Set the exec context.
                require!(
                    self.exec_context.is_none(),
                    ClockworkError::InvalidThreadState
                );
                self.exec_context = Some(ExecContext {
                    execs_since_reimbursement: 0,
                    execs_since_slot: 0,
                    last_exec_at: clock.slot,
                    trigger_context: TriggerContext::Immediate,
                });
            }
        }

        // If we make it here, the trigger is active. Update the next instruction and be done.
        self.next_instruction = Some(self.kickoff_instruction.clone());

        // Realloc the thread account
        self.realloc()?;

        Ok(())
    }
}

/// The triggering conditions of a thread.
#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, PartialEq)]
pub enum Trigger {
    /// Allows a thread to be kicked off whenever the data of an account changes.
    Account {
        /// The address of the account to monitor.
        address: Pubkey,
        /// The byte offset of the account data to monitor.
        offset: usize,
        /// The size of the byte slice to monitor (must be less than 1kb)
        size: usize,
    },

    /// Allows a thread to be kicked off according to a one-time or recurring schedule.
    Cron {
        /// The schedule in cron syntax. Value must be parsable by the `clockwork_cron` package.
        schedule: String,

        /// Boolean value indicating whether triggering moments may be skipped if they are missed (e.g. due to network downtime).
        /// If false, any "missed" triggering moments will simply be executed as soon as the network comes back online.
        skippable: bool,
    },

    /// Allows a thread to be kicked off as soon as it's created.
    Immediate,
}

/// The execution context of a particular transaction thread.
#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ExecContext {
    /// Number of execs since the last tx reimbursement.
    pub execs_since_reimbursement: u64,

    /// Number of execs in this slot.
    pub execs_since_slot: u64,

    /// Slot of the last exec
    pub last_exec_at: u64,

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
