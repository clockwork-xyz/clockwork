use {
    crate::state::*,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(schedule: String)]
pub struct QueueUpdate<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.authority.key().as_ref(),
            queue.name.as_bytes(),
        ],
        bump,
        has_one = authority,
        constraint = queue.status == QueueStatus::Pending || queue.status == QueueStatus::Paused
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<QueueUpdate>, schedule: String) -> Result<()> {
    // Get accounts
    let queue = &mut ctx.accounts.queue;

    // Update the queue
    let ts = Clock::get().unwrap().unix_timestamp;
    queue.schedule = schedule;
    queue.process_at = queue.next_process_at(ts);

    Ok(())
}
