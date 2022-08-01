use {
    crate::{state::*, errors::ClockworkError},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct QueueProcess<'info> {
    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.authority.as_ref(),
            queue.name.as_bytes(),
        ],
        bump,
        constraint = queue.process_at.is_some() && queue.process_at <= Some(Clock::get().unwrap().unix_timestamp) @ ClockworkError::QueueNotDue,
        constraint = queue.status == QueueStatus::Pending @ ClockworkError::InvalidQueueStatus,
    )]
    pub queue: Account<'info, Queue>,

    #[account(mut)]
    pub worker: Signer<'info>,
}

pub fn handler(ctx: Context<QueueProcess>) -> Result<()> {
    let queue = &mut ctx.accounts.queue;
    queue.process()
}
