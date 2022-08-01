use {
    crate::state::*,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct QueuePause<'info> {
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
        constraint = queue.status == QueueStatus::Pending
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<QueuePause>) -> Result<()> {
    // Get accounts
    let queue = &mut ctx.accounts.queue;

    // Pause the queue
    queue.status = QueueStatus::Paused;

    Ok(())
}
