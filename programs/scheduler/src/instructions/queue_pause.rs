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
            queue.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = authority,
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
