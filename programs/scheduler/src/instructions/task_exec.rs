use {
    crate::{events::*, state::*, errors::CronosError},
    anchor_lang::{prelude::*, solana_program::sysvar},
};

#[derive(Accounts)]
pub struct TaskExec<'info> {
    #[account(
        mut,
        seeds = [
            SEED_ACTION,
            action.task.as_ref(),
            action.id.to_be_bytes().as_ref()
        ],
        bump,
    )]
    pub action: Account<'info, Action>,

    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [SEED_CONFIG],
        bump,
    )]
    pub config: Account<'info, Config>,

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

    #[account(
        mut,
        seeds = [
            SEED_QUEUE,
            queue.owner.as_ref()
        ],
        bump = queue.bump,
    )]
    pub queue: Account<'info, Queue>,

    #[account(
        mut,
        seeds = [
            SEED_TASK, 
            task.queue.as_ref(),
            task.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = queue,
        constraint = task.exec_at.is_some() && task.exec_at <= Some(clock.unix_timestamp) @ CronosError::TaskNotDue,
        constraint = match task.status {
            TaskStatus::Executing { action_id } => action_id == action.id,
            _ => false,
        } @ CronosError::InvalidTaskStatus
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(ctx: Context<TaskExec>) -> Result<()> {
    let action = &mut ctx.accounts.action;
    let clock = &ctx.accounts.clock;
    let config = &ctx.accounts.config;
    let delegate = &mut ctx.accounts.delegate;
    let fee = &mut ctx.accounts.fee;
    let queue = &mut ctx.accounts.queue;
    let task = &mut ctx.accounts.task;

    task.exec(&ctx.remaining_accounts.iter().as_slice(), action, delegate, config, fee, queue)?;

    emit!(TaskExecuted {
        delegate: delegate.key(),
        task: task.key(),
        ts: clock.unix_timestamp,
    });

    Ok(())
}
