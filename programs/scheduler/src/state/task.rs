use {
    super::{Action, Config, Fee, Queue, QueueAccount},
    crate::{errors::CronosError, pda::PDA},
    anchor_lang::{prelude::*, solana_program::instruction::Instruction, AnchorDeserialize},
    chrono::{DateTime, NaiveDateTime, Utc},
    cronos_cron::Schedule,
    std::{convert::TryFrom, str::FromStr},
};

pub const SEED_TASK: &[u8] = b"task";

/**
 * Task
 */

#[account]
#[derive(Debug)]
pub struct Task {
    pub action_count: u128,
    pub exec_at: Option<i64>,
    pub id: u128,
    pub queue: Pubkey,
    pub schedule: String,
    pub status: TaskStatus,
}

impl Task {
    pub fn pda(queue: Pubkey, id: u128) -> PDA {
        Pubkey::find_program_address(
            &[SEED_TASK, queue.as_ref(), id.to_be_bytes().as_ref()],
            &crate::ID,
        )
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
    fn new(
        &mut self,
        clock: &Sysvar<Clock>,
        queue: &mut Account<Queue>,
        schedule: String,
    ) -> Result<()>;

    fn cancel(&mut self, to: &mut Signer) -> Result<()>;

    fn exec(
        &mut self,
        account_infos: &[AccountInfo],
        action: &mut Account<Action>,
        delegate: &mut Signer,
        config: &Account<Config>,
        fee: &mut Account<Fee>,
        queue: &mut Account<Queue>,
    ) -> Result<()>;

    fn start(&mut self) -> Result<()>;

    fn finish(&mut self) -> Result<()>;

    fn next_exec_at(&self, ts: i64) -> Option<i64>;

    fn roll_forward(&mut self) -> Result<()>;
}

impl TaskAccount for Account<'_, Task> {
    fn new(
        &mut self,
        clock: &Sysvar<Clock>,
        queue: &mut Account<Queue>,
        schedule: String,
    ) -> Result<()> {
        // Initialize task account
        self.action_count = 0;
        self.id = queue.task_count;
        self.queue = queue.key();
        self.schedule = schedule;
        self.status = TaskStatus::Pending;

        // Set exec_at (schedule must be set first)
        self.exec_at = self.next_exec_at(clock.unix_timestamp);

        // Increment queue task counter
        queue.task_count = queue.task_count.checked_add(1).unwrap();

        Ok(())
    }

    fn cancel(&mut self, to: &mut Signer) -> Result<()> {
        let lamports = self.to_account_info().lamports();
        **self.to_account_info().try_borrow_mut_lamports()? = self
            .to_account_info()
            .lamports()
            .checked_sub(lamports)
            .unwrap();
        **to.to_account_info().try_borrow_mut_lamports()? = to
            .to_account_info()
            .lamports()
            .checked_add(lamports)
            .unwrap();
        Ok(())
    }

    fn exec(
        &mut self,
        account_infos: &[AccountInfo],
        action: &mut Account<Action>,
        delegate: &mut Signer,
        config: &Account<Config>,
        fee: &mut Account<Fee>,
        queue: &mut Account<Queue>,
    ) -> Result<()> {
        // Validate the action id matches the task's current execution state
        require!(
            action.id
                == match self.status {
                    TaskStatus::Executing { action_id } => action_id,
                    _ => return Err(CronosError::InvalidTaskStatus.into()),
                },
            CronosError::InvalidAction
        );

        // Record the delegate's lamports before invoking inner ixs
        let delegate_lamports_pre = delegate.lamports();

        // Process all of the action instructions
        for ix in &action.ixs {
            // TODO Verify account_infos matches the metadata in ix.accounts
            //      for account in ix.accounts {}

            for account_info in account_infos {
                // Attack vector: Consider actions where the ixs contain
                //  mutable accounts owned by the scheduler program.
                //  Inner ixs are signed by the queue PDA.
                //  What accounts can these inner instructions mutate?
                if *account_info.owner == crate::ID {
                    // TODO Given the queue is a signer, should any mutable accounts
                    //      owned by this program be passed into an inner ix –
                    //      written by a presumably malicious 3rd party?
                    //
                    // If any this program's accounts are allowed into inner ixs –
                    //  for example, to allow for updating a task or action –
                    //  what verification checks are needed and what modifications will be allowed?
                    //
                    // DO allow any read-only accounts owned by this program into the inner ix.
                    // DO NOT allow mutable accounts owned by this program into the inner ix.
                    // The ONLY allowed mutable account should be *this* action account.
                    // After the executing inner instructions, the only thing that is allowed
                    //  to have changed is the set of inner ixs.
                }

                // TODO Create a unique payer address (ie CronPayer111111111111111111111)
                //      If an account matches this address, then replace it with the delegate address.
                //      This will provide a way to inject a "payer" address into inner ixs.
                //      This is necessary to allow inner ixs to init new accounts.
                //
                // Consider the security implications to delegates. They will be a signer
                //  on arbitrary ixs written by malicious third parties. Delegates should not
                //  hold *any* assets on these wallets other than SOL.
            }
            queue.sign(&Instruction::from(ix), account_infos)?;
        }

        // Track how many lamports the delegate lost
        let delegate_lamports_post = delegate.lamports();
        let delegate_reimbursement = delegate_lamports_pre
            .checked_sub(delegate_lamports_post)
            .unwrap();

        // Pay delegate fees
        let total_delegate_fee = config
            .delegate_fee
            .checked_add(delegate_reimbursement)
            .unwrap();
        **queue.to_account_info().try_borrow_mut_lamports()? = queue
            .to_account_info()
            .lamports()
            .checked_sub(total_delegate_fee)
            .unwrap();
        **delegate.to_account_info().try_borrow_mut_lamports()? = delegate
            .to_account_info()
            .lamports()
            .checked_add(total_delegate_fee)
            .unwrap();

        // Pay program fees
        **queue.to_account_info().try_borrow_mut_lamports()? = queue
            .to_account_info()
            .lamports()
            .checked_sub(config.program_fee)
            .unwrap();
        **fee.to_account_info().try_borrow_mut_lamports()? = fee
            .to_account_info()
            .lamports()
            .checked_add(config.program_fee)
            .unwrap();

        // Increment collectable fee balance
        fee.balance = fee.balance.checked_add(config.program_fee).unwrap();

        // Update the task status
        let next_action_id = action.id.checked_add(1).unwrap();
        if next_action_id == self.action_count {
            self.finish()?;
        } else {
            self.status = TaskStatus::Executing {
                action_id: next_action_id,
            };
        }

        Ok(())
    }

    fn start(&mut self) -> Result<()> {
        // Validate the task is pending
        require!(
            self.status == TaskStatus::Pending,
            CronosError::InvalidTaskStatus,
        );

        if self.action_count > 0 {
            // If there are actions, change the task status to 'executing'
            self.status = TaskStatus::Executing { action_id: 0 };
        } else {
            // Otherwise, just roll forward the exec_at timestamp
            self.roll_forward()?;
        }

        Ok(())
    }

    fn finish(&mut self) -> Result<()> {
        self.status = TaskStatus::Pending;
        self.roll_forward()
    }

    fn next_exec_at(&self, ts: i64) -> Option<i64> {
        match Schedule::from_str(&self.schedule)
            .unwrap()
            .after(&DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp(ts, 0),
                Utc,
            ))
            .take(1)
            .next()
        {
            Some(datetime) => Some(datetime.timestamp()),
            None => None,
        }
    }

    fn roll_forward(&mut self) -> Result<()> {
        match self.exec_at {
            Some(exec_at) => self.exec_at = self.next_exec_at(exec_at),
            None => (),
        };
        Ok(())
    }
}

/**
 * TaskStatus
 */

#[derive(AnchorDeserialize, AnchorSerialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TaskStatus {
    Executing { action_id: u128 },
    Paused,
    Pending,
}
