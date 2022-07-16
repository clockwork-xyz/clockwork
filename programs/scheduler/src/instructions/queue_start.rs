use {
    crate::{state::*, errors::CronosError},
    anchor_lang::{prelude::*, solana_program::sysvar},
};

#[derive(Accounts)]
pub struct QueueStart<'info> {
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [
            SEED_DELEGATE,
            delegate.authority.as_ref()
        ],
        bump,
    )]
    pub delegate: Account<'info, Delegate>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.delegate.as_ref(),
            queue.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = delegate,
        constraint = queue.exec_at.is_some() && queue.exec_at <= Some(clock.unix_timestamp) @ CronosError::QueueNotDue,
        constraint = queue.status == QueueStatus::Pending @ CronosError::InvalidQueueStatus,
    )]
    pub queue: Account<'info, Queue>,

    #[account(mut)]
    pub worker: Signer<'info>,
}

pub fn handler(ctx: Context<QueueStart>) -> Result<()> {
    let queue = &mut ctx.accounts.queue;
    queue.start()
}
