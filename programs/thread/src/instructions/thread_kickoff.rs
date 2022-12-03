use {
    crate::{errors::*, state::*},
    anchor_lang::prelude::*,
    clockwork_network_program::state::{Worker, WorkerAccount},
};

/// Accounts required by the `thread_kickoff` instruction.
#[derive(Accounts)]
pub struct ThreadKickoff<'info> {
    /// The signatory.
    #[account(mut)]
    pub signatory: Signer<'info>,

    /// The thread to kickoff.
    #[account(
        mut,
        seeds = [
            SEED_THREAD,
            thread.authority.as_ref(),
            thread.id.as_bytes(),
        ],
        bump,
        constraint = !thread.paused @ ClockworkError::ThreadPaused,
        constraint = thread.next_instruction.is_none() @ ClockworkError::ThreadBusy,
    )]
    pub thread: Box<Account<'info, Thread>>,

    /// The worker.
    #[account(address = worker.pubkey())]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<ThreadKickoff>) -> Result<()> {
    // Get accounts.
    let thread = &mut ctx.accounts.thread;

    // If this thread does not have a next_instruction, verify the thread's trigger condition is active.
    thread.kickoff(ctx.remaining_accounts)?;

    Ok(())
}
