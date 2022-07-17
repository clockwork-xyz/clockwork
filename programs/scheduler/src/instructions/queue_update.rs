use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::sysvar},
};

#[derive(Accounts)]
#[instruction(schedule: String)]
pub struct QueueUpdate<'info> {
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
        constraint = queue.status == QueueStatus::Pending || queue.status == QueueStatus::Paused
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<QueueUpdate>, schedule: String) -> Result<()> {
    // Get accounts
    let clock = &ctx.accounts.clock;
    let queue = &mut ctx.accounts.queue;

    // Update the queue
    queue.schedule = schedule;
    queue.process_at = queue.next_process_at(clock.unix_timestamp);

    Ok(())
}
