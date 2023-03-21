use {crate::state::*, anchor_lang::prelude::*};

/// Accounts required by the `thread_reset` instruction.
#[derive(Accounts)]
pub struct ThreadReset<'info> {
    /// The authority (owner) of the thread.
    #[account()]
    pub authority: Signer<'info>,

    /// The thread to be paused.
    #[account(
        mut,
        seeds = [
            SEED_THREAD,
            thread.authority.as_ref(),
            thread.id.as_slice(),
        ],
        bump = thread.bump,
        has_one = authority
    )]
    pub thread: Account<'info, Thread>,
}

pub fn handler(ctx: Context<ThreadReset>) -> Result<()> {
    // Get accounts
    let thread = &mut ctx.accounts.thread;

    // Full reset the thread state.
    thread.next_instruction = None;
    thread.exec_context = None;
    thread.created_at = Clock::get().unwrap().into();

    Ok(())
}
