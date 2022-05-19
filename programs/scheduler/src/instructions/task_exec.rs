use {
    crate::{errors::CronosError, events::TaskExecuted, state::*},
    anchor_lang::{prelude::*, solana_program::sysvar},
};

#[derive(Accounts)]
pub struct TaskExec<'info> {
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
            queue.key().as_ref()
        ],
        bump,
        has_one = queue
    )]
    pub fee: Account<'info, Fee>,

    #[account(seeds = [SEED_MANAGER, manager.authority.as_ref()], bump)]
    pub manager: Account<'info, Manager>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.manager.as_ref(),
            queue.id.to_be_bytes().as_ref(),
        ],
        bump,
        constraint = queue.exec_at.is_some() && queue.exec_at <= Some(clock.unix_timestamp) @ CronosError::QueueNotDue,
        constraint = match queue.status {
            QueueStatus::Executing { task_id } => task_id == task.id,
            _ => false,
        } @ CronosError::InvalidQueueStatus
    )]
    pub queue: Account<'info, Queue>,

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
}

pub fn handler(ctx: Context<TaskExec>) -> Result<()> {
    let task = &mut ctx.accounts.task;
    let clock = &ctx.accounts.clock;
    let config = &ctx.accounts.config;
    let delegate = &mut ctx.accounts.delegate;
    let fee = &mut ctx.accounts.fee;
    let manager = &ctx.accounts.manager;
    let queue = &mut ctx.accounts.queue;

    let account_infos = &mut ctx.remaining_accounts.clone().to_vec();

    let manager_bump = *ctx.bumps.get("manager").unwrap();
    task.exec(account_infos, config, delegate, fee, manager, manager_bump, queue)?;

    emit!(TaskExecuted {
        delegate: delegate.key(),
        queue: queue.key(),
        task: task.key(),
        ts: clock.unix_timestamp,
    });

    Ok(())
}
