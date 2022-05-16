use {
    crate::{state::*, errors::CronosError},
    anchor_lang::{prelude::*, solana_program::sysvar},
};

#[derive(Accounts)]
pub struct QueueBegin<'info> {
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(mut)]
    pub delegate: Signer<'info>,

    #[account(
        seeds = [
            SEED_MANAGER,
            manager.owner.as_ref()
        ],
        bump = manager.bump,
    )]
    pub manager: Account<'info, Manager>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.manager.as_ref(),
            queue.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = manager,
        constraint = queue.exec_at.is_some() && queue.exec_at <= Some(clock.unix_timestamp) @ CronosError::QueueNotDue,
        constraint = queue.status == QueueStatus::Pending @ CronosError::InvalidQueueStatus,
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<QueueBegin>) -> Result<()> {
    let queue = &mut ctx.accounts.queue;
    queue.begin()
}
