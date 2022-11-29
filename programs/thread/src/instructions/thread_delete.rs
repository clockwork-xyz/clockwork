use {crate::state::*, anchor_lang::prelude::*};

/// Accounts required by the `thread_delete` instruction.
#[derive(Accounts)]
pub struct ThreadDelete<'info> {
    /// The authority (owner) of the thread.
    #[account()]
    pub authority: Signer<'info>,

    /// The address to return the data rent lamports to.
    #[account(mut)]
    pub close_to: SystemAccount<'info>,

    /// The thread to be delete.
    #[account(
        mut,
        seeds = [
            SEED_THREAD,
            thread.authority.as_ref(),
            thread.id.as_bytes(),
        ],
        bump,
        has_one = authority,
        close = close_to
    )]
    pub thread: Account<'info, Thread>,
}

pub fn handler(_ctx: Context<ThreadDelete>) -> Result<()> {
    Ok(())
}
