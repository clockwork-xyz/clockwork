use crate::state::FeeAccount;

use {
    super::{Config, Fee, Queue},
    crate::{
        errors::ClockworkError,
        state::{QueueAccount, QueueStatus},
    },
    anchor_lang::{
        prelude::borsh::BorshSchema, prelude::*, solana_program::instruction::Instruction,
        AnchorDeserialize,
    },
    std::convert::TryFrom,
};

pub const SEED_TASK: &[u8] = b"task";

/**
 * Task
 */

#[account]
#[derive(Debug)]
pub struct Task {
    pub id: u64,
    pub ixs: Vec<InstructionData>,
    pub queue: Pubkey,
}

impl Task {
    pub fn pubkey(queue: Pubkey, id: u64) -> Pubkey {
        Pubkey::find_program_address(
            &[SEED_TASK, queue.as_ref(), id.to_be_bytes().as_ref()],
            &crate::ID,
        )
        .0
    }
}

impl TryFrom<Vec<u8>> for Task {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Task::try_deserialize(&mut data.as_slice())
    }
}

/**
 * TaskAccount
 */

pub trait TaskAccount {
    fn new(&mut self, ixs: Vec<InstructionData>, queue: &mut Account<Queue>) -> Result<()>;

    fn exec(
        &mut self,
        account_infos: &Vec<AccountInfo>,
        config: &Account<Config>,
        fee: &mut Account<Fee>,
        queue: &mut Account<Queue>,
        queue_bump: u8,
        worker: &mut Signer,
    ) -> Result<()>;
}

impl TaskAccount for Account<'_, Task> {
    fn new(&mut self, ixs: Vec<InstructionData>, queue: &mut Account<Queue>) -> Result<()> {
        // Reject inner instructions if they have a signer other than the queue or payer
        for ix in ixs.iter() {
            for acc in ix.accounts.iter() {
                if acc.is_signer {
                    require!(
                        acc.pubkey == queue.key() || acc.pubkey == crate::payer::ID,
                        ClockworkError::InvalidSignatory
                    );
                }
            }
        }

        // Save data
        self.id = queue.task_count;
        self.ixs = ixs;
        self.queue = queue.key();

        // Increment the queue's task count
        queue.task_count = queue.task_count.checked_add(1).unwrap();

        Ok(())
    }

    fn exec(
        &mut self,
        account_infos: &Vec<AccountInfo>,
        config: &Account<Config>,
        fee: &mut Account<Fee>,
        queue: &mut Account<Queue>,
        queue_bump: u8,
        worker: &mut Signer,
    ) -> Result<()> {
        // Validate the task id matches the queue's current execution state
        require!(
            self.id
                == match queue.status {
                    QueueStatus::Processing { task_id } => task_id,
                    _ => return Err(ClockworkError::InvalidQueueStatus.into()),
                },
            ClockworkError::InvalidTask
        );

        // Validate the worker data is empty
        require!(worker.data_is_empty(), ClockworkError::WorkerDataNotEmpty);

        // Record the worker's lamports before invoking inner ixs
        let worker_lamports_pre = worker.lamports();

        // Create an array of dynamic ixs to update the task for the next invocation
        let dyanmic_ixs: &mut Vec<InstructionData> = &mut vec![];

        // Process all of the task instructions
        for ix in &self.ixs {
            // If an inner ix account matches the Clockwork payer address (ClockworkPayer11111111111111111111111111111111),
            //  then inject the worker in its place. Dapp developers can use the worker as a payer to initialize
            //  new accounts in their tasks. Workers will be reimbursed for all SOL spent during the inner ixs.
            //
            // Because the worker can be injected as the signer on inner ixs (written by presumed malicious parties),
            //  node operators should not secure any assets or staking positions with their worker wallets other than
            //  an operational level of lamports needed to submit txns (~0.01 ⊚).
            let accs: &mut Vec<AccountMetaData> = &mut vec![];
            ix.accounts.iter().for_each(|acc| {
                if acc.pubkey == crate::payer::ID {
                    accs.push(AccountMetaData {
                        pubkey: worker.key(),
                        is_signer: acc.is_signer,
                        is_writable: acc.is_writable,
                    });
                } else {
                    accs.push(acc.clone());
                }
            });

            // Execute the inner ix and process the response. Note that even though the queue PDA is a signer
            //  on this ix, Solana will not allow downstream programs to mutate accounts owned by this program
            //  and explicitly forbids CPI reentrancy.
            let exec_response = queue.sign(
                &account_infos,
                queue_bump,
                &InstructionData {
                    program_id: ix.program_id,
                    accounts: accs.clone(),
                    data: ix.data.clone(),
                },
            )?;

            // Process the exec response
            match exec_response {
                None => (),
                Some(exec_response) => match exec_response.dynamic_accounts {
                    None => (),
                    Some(dynamic_accounts) => {
                        require!(
                            dynamic_accounts.len() == ix.accounts.len(),
                            ClockworkError::InvalidDynamicAccounts
                        );
                        dyanmic_ixs.push(InstructionData {
                            program_id: ix.program_id,
                            accounts: dynamic_accounts
                                .iter()
                                .enumerate()
                                .map(|(i, pubkey)| {
                                    let acc = ix.accounts.get(i).unwrap();
                                    AccountMetaData {
                                        pubkey: match pubkey {
                                            _ if *pubkey == worker.key() => crate::payer::ID,
                                            _ => *pubkey,
                                        },
                                        is_signer: acc.is_signer,
                                        is_writable: acc.is_writable,
                                    }
                                })
                                .collect::<Vec<AccountMetaData>>(),
                            data: ix.data.clone(),
                        });
                    }
                },
            }
        }

        // Verify that inner ixs have not initialized data at the worker address
        require!(worker.data_is_empty(), ClockworkError::WorkerDataNotEmpty);

        // Update the actions's ixs for the next invocation
        if !dyanmic_ixs.is_empty() {
            self.ixs = dyanmic_ixs.clone();
        }

        // Track how many lamports the worker spent in the inner ixs
        let worker_lamports_post = worker.lamports();
        let worker_reimbursement = worker_lamports_pre
            .checked_sub(worker_lamports_post)
            .unwrap();

        // Pay worker fees
        let total_worker_fee = config.worker_fee.checked_add(worker_reimbursement).unwrap();
        fee.pay_to_worker(total_worker_fee, queue)?;

        // Update the queue status
        let next_task_id = self.id.checked_add(1).unwrap();
        if next_task_id == queue.task_count {
            queue.roll_forward()?;
        } else {
            queue.status = QueueStatus::Processing {
                task_id: next_task_id,
            };
        }

        Ok(())
    }
}

/**
 * InstructionData
 */

#[derive(AnchorDeserialize, AnchorSerialize, BorshSchema, Clone, Debug, PartialEq)]
pub struct InstructionData {
    /// Pubkey of the instruction processor that executes this instruction
    pub program_id: Pubkey,
    /// Metadata for what accounts should be passed to the instruction processor
    pub accounts: Vec<AccountMetaData>,
    /// Opaque data passed to the instruction processor
    pub data: Vec<u8>,
}

impl From<Instruction> for InstructionData {
    fn from(instruction: Instruction) -> Self {
        InstructionData {
            program_id: instruction.program_id,
            accounts: instruction
                .accounts
                .iter()
                .map(|a| AccountMetaData {
                    pubkey: a.pubkey,
                    is_signer: a.is_signer,
                    is_writable: a.is_writable,
                })
                .collect(),
            data: instruction.data,
        }
    }
}

impl From<&InstructionData> for Instruction {
    fn from(instruction: &InstructionData) -> Self {
        Instruction {
            program_id: instruction.program_id,
            accounts: instruction
                .accounts
                .iter()
                .map(|a| AccountMeta {
                    pubkey: a.pubkey,
                    is_signer: a.is_signer,
                    is_writable: a.is_writable,
                })
                .collect(),
            data: instruction.data.clone(),
        }
    }
}

impl TryFrom<Vec<u8>> for InstructionData {
    type Error = Error;
    fn try_from(data: Vec<u8>) -> std::result::Result<Self, Self::Error> {
        Ok(
            borsh::try_from_slice_with_schema::<InstructionData>(data.as_slice())
                .map_err(|_err| ErrorCode::AccountDidNotDeserialize)?,
        )
    }
}

/**
 * AccountMetaData
 */

#[derive(AnchorDeserialize, AnchorSerialize, BorshSchema, Clone, Debug, PartialEq)]
pub struct AccountMetaData {
    /// An account's public key
    pub pubkey: Pubkey,
    /// True if an Instruction requires a Transaction signature matching `pubkey`.
    pub is_signer: bool,
    /// True if the `pubkey` can be loaded as a read-write account.
    pub is_writable: bool,
}
