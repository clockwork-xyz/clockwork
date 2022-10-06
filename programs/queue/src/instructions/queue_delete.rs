use {crate::objects::*, anchor_lang::prelude::*};

/// Accounts required by the `queue_delete` instruction.
#[derive(Accounts)]
pub struct QueueDelete<'info> {
    /// The authority (owner) of the queue.
    #[account()]
    pub authority: Signer<'info>,

    /// The address to return the data rent lamports to.
    #[account(mut)]
    pub close_to: SystemAccount<'info>,

    /// The queue to be delete.
    #[account(
        mut,
        seeds = [
            SEED_QUEUE,
            queue.authority.as_ref(),
            queue.id.as_bytes(),
        ],
        bump,
        has_one = authority,
        close = close_to
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(_ctx: Context<QueueDelete>) -> Result<()> {
    Ok(())
}
