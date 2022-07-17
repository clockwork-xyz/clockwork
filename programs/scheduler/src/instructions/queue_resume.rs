use {
    crate::state::*,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(skip_forward: bool)]
pub struct QueueResume<'info> {
    #[account()]
    pub authority: Signer<'info>,

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
    let queue = &mut ctx.accounts.queue;

    // Skip forward, if required
    if skip_forward {
        let ts = Clock::get().unwrap().unix_timestamp;
        queue.process_at = queue.next_process_at(ts);
    }

    // Pause the queue
    queue.status = QueueStatus::Pending;

    Ok(())
}
