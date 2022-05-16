use {
    crate::{errors::CronosError, events::QueueExecuted, state::*},
    anchor_lang::{prelude::*, solana_program::sysvar},
};

#[derive(Accounts)]
pub struct QueueExec<'info> {
    #[account(
        mut,
        seeds = [
            SEED_TASK,
            task.queue.as_ref(),
            task.id.to_be_bytes().as_ref()
        ],
        bump,
    )]
    pub task: Account<'info, Task>,

    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(mut)]
    pub delegate: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_FEE,
            yogi.key().as_ref()
        ],
        bump,
        has_one = yogi
    )]
    pub fee: Account<'info, Fee>,

    #[account(
        seeds = [
            SEED_YOGI,
            yogi.owner.as_ref()
        ],
        bump,
    )]
    pub yogi: Box<Account<'info, Yogi>>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.yogi.as_ref(),
            queue.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = yogi,
        constraint = queue.exec_at.is_some() && queue.exec_at <= Some(clock.unix_timestamp) @ CronosError::QueueNotDue,
        constraint = match queue.status {
            QueueStatus::Executing { task_id } => task_id == task.id,
            _ => false,
        } @ CronosError::InvalidQueueStatus
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<QueueExec>) -> Result<()> {
    let task = &mut ctx.accounts.task;
    let clock = &ctx.accounts.clock;
    let config = &ctx.accounts.config;
    let delegate = &mut ctx.accounts.delegate;
    let fee = &mut ctx.accounts.fee;
    let yogi = &ctx.accounts.yogi;
    let queue = &mut ctx.accounts.queue;

    let remaining_accounts = &mut ctx.remaining_accounts.clone().to_vec();

    queue.exec(remaining_accounts, task, delegate, config, fee, yogi)?;

    emit!(QueueExecuted {
        delegate: delegate.key(),
        queue: queue.key(),
        ts: clock.unix_timestamp,
    });

    Ok(())
}
