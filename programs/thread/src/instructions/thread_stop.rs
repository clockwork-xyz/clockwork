use {crate::state::*, anchor_lang::prelude::*};

/// Accounts required by the `thread_delete` instruction.
#[derive(Accounts)]
pub struct ThreadStop<'info> {
    /// The authority (owner) of the thread.
    #[account()]
    pub authority: Signer<'info>,

    /// The thread to be paused.
    #[account(
        mut,
        seeds = [
            SEED_THREAD,
            thread.authority.as_ref(),
            thread.id.as_bytes(),
        ],
        bump,
        has_one = authority
    )]
    pub thread: Account<'info, Thread>,
}

pub fn handler(ctx: Context<ThreadStop>) -> Result<()> {
    // Get accounts
    let thread = &mut ctx.accounts.thread;

    // Pause the thread
    thread.next_instruction = None;

    Ok(())
}
