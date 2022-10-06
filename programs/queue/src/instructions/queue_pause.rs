use {crate::objects::*, anchor_lang::prelude::*};

/// Accounts required by the `queue_delete` instruction.
#[derive(Accounts)]
pub struct QueuePause<'info> {
    /// The authority (owner) of the queue.
    #[account()]
    pub authority: Signer<'info>,

    /// The queue to be paused.
    #[account(
        mut,
        seeds = [
            SEED_QUEUE,
            queue.authority.as_ref(),
            queue.id.as_bytes(),
        ],
        bump,
        has_one = authority
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<QueuePause>) -> Result<()> {
    // Get accounts
    let queue = &mut ctx.accounts.queue;

    // Pause the queue
    queue.paused = true;

    Ok(())
}
