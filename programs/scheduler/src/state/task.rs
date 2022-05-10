use {
    super::{Action, Config, Fee, Queue, QueueAccount},
    crate::pda::PDA,
    anchor_lang::{prelude::*, solana_program::instruction::Instruction, AnchorDeserialize},
    chrono::{DateTime, NaiveDateTime, Utc},
    cronos_cron::Schedule,
    std::{collections::HashSet, convert::TryFrom, str::FromStr},
};

pub const SEED_TASK: &[u8] = b"task";

/**
 * Task
 */

#[account]
#[derive(Debug)]
pub struct Task {
    pub action_count: u128,
    pub bump: u8,
    pub delegates: HashSet<Pubkey>,
    pub exec_at: Option<i64>,
    pub id: u128,
    pub queue: Pubkey,
    pub schedule: String,
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
        bump: u8,
        clock: &Sysvar<Clock>,
        queue: &mut Account<Queue>,
        schedule: String,
    ) -> Result<()>;

    fn cancel(&mut self, to: &mut Signer) -> Result<()>;

    fn exec(
        &mut self,
        account_infos: &[AccountInfo],
        action: &mut Account<Action>,
        bot: &mut Signer,
        config: &Account<Config>,
        fee: &mut Account<Fee>,
        queue: &mut Account<Queue>,
    ) -> Result<()>;

    fn next_exec_at(&self, ts: i64) -> Option<i64>;
}

impl TaskAccount for Account<'_, Task> {
    fn new(
        &mut self,
        bump: u8,
        clock: &Sysvar<Clock>,
        queue: &mut Account<Queue>,
        schedule: String,
    ) -> Result<()> {
        // Initialize task account.
        self.action_count = 0;
        self.bump = bump;
        self.id = queue.task_count;
        self.queue = queue.key();
        self.schedule = schedule;

        // Move forward, one step in time
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
        bot: &mut Signer,
        config: &Account<Config>,
        fee: &mut Account<Fee>,
        queue: &mut Account<Queue>,
    ) -> Result<()> {
        // Sign all of the action instructions
        for ix in &action.ixs {
            queue.sign(&Instruction::from(ix), account_infos)?;
        }

        // Update the exec_at timestamp
        match self.exec_at {
            Some(exec_at) => self.exec_at = self.next_exec_at(exec_at),
            None => {}
        }

        // Pay automation fees
        **queue.to_account_info().try_borrow_mut_lamports()? = queue
            .to_account_info()
            .lamports()
            .checked_sub(config.program_fee)
            .unwrap();
        **bot.to_account_info().try_borrow_mut_lamports()? = bot
            .to_account_info()
            .lamports()
            .checked_add(config.program_fee)
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

        // Increment collectable fee balance.
        fee.balance = fee.balance.checked_add(config.program_fee).unwrap();

        Ok(())
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
}
