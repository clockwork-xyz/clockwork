use {
    crate::{state::*, errors::CronosError},
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
        constraint = queue.process_at.is_some() && queue.process_at <= Some(Clock::get().unwrap().unix_timestamp) @ CronosError::QueueNotDue,
        constraint = queue.status == QueueStatus::Pending @ CronosError::InvalidQueueStatus,
    )]
    pub queue: Account<'info, Queue>,

    #[account(mut)]
    pub worker: Signer<'info>,
}

pub fn handler(ctx: Context<QueueProcess>) -> Result<()> {
    let queue = &mut ctx.accounts.queue;
    queue.process()
}
