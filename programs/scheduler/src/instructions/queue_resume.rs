use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::sysvar},
};

#[derive(Accounts)]
#[instruction(skip_forward: bool)]
pub struct QueueResume<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.authority.key().as_ref(),
            queue.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = authority,
        constraint = queue.status == QueueStatus::Paused
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<QueueResume>, skip_forward: bool) -> Result<()> {
    // Get accounts
    let clock = &ctx.accounts.clock;
    let queue = &mut ctx.accounts.queue;

    // Skip forward, if required
    if skip_forward {
        queue.exec_at = queue.next_exec_at(clock.unix_timestamp);
    }

    // Pause the queue
    queue.status = QueueStatus::Pending;

    Ok(())
}
