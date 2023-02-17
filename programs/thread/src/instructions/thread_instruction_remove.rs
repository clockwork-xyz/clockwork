use {crate::state::*, anchor_lang::prelude::*};

/// Accounts required by the `thread_instruction_remove` instruction.
#[derive(Accounts)]
#[instruction(index: u64)]
pub struct ThreadInstructionRemove<'info> {
    /// The authority (owner) of the thread.
    #[account()]
    pub authority: Signer<'info>,

    /// The thread to be edited.
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

pub fn handler(ctx: Context<ThreadInstructionRemove>, index: u64) -> Result<()> {
    // Get accounts
    let thread = &mut ctx.accounts.thread;

    // Pause the thread
    thread.instructions.remove(index as usize);

    Ok(())
}
