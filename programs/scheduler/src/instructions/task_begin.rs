use {
    crate::{state::*, errors::CronosError},
    anchor_lang::{prelude::*, solana_program::sysvar},
};

#[derive(Accounts)]
pub struct TaskBegin<'info> {
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(mut)]
    pub delegate: Signer<'info>,

    #[account(
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
        constraint = task.status == TaskStatus::Pending @ CronosError::InvalidTaskStatus,
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(ctx: Context<TaskBegin>) -> Result<()> {
    let task = &mut ctx.accounts.task;
    task.begin()
}
