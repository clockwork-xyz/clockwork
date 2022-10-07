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
    clockwork_utils::*,
    std::{
        convert::TryFrom,
        hash::{Hash, Hasher},
    },
};

// TODO Add support for lookup tables.
//      If the value is set, then use that lookup table when building the transaction.
//      Add a property to CrankResponse to allow updating the lookup table.
//      I believe Transaction.v0 only supports one lookup table at a time. So if this value changes between cranks,
//      workers will need to stop packing the transaction and submit.

pub const SEED_QUEUE: &[u8] = b"queue";

const DEFAULT_RATE_LIMIT: u64 = 10;

const MINIMUM_FEE: u64 = 1000;

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
    fn crank(&mut self, account_infos: &[AccountInfo], bump: u8, worker: &Signer) -> Result<()>;

    /// Reallocate the memory allocation for the account.
    fn realloc(&mut self) -> Result<()>;
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

    fn crank(&mut self, account_infos: &[AccountInfo], bump: u8, signatory: &Signer) -> Result<()> {
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

        Ok(())
    }

    fn realloc(&mut self) -> Result<()> {
        // Realloc memory for the queue account
        let data_len = 8 + self.try_to_vec()?.len();
        self.to_account_info().realloc(data_len, false)?;
        Ok(())
    }
}

/// The triggering conditions of a queue.
#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone)]
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
