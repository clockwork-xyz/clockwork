use {crate::state::*, anchor_lang::prelude::*};

/// Accounts required by the `thread_instruction_add` instruction.
#[derive(Accounts)]
#[instruction(instruction: SerializableInstruction)]
pub struct ThreadInstructionAdd<'info> {
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

pub fn handler(
    ctx: Context<ThreadInstructionAdd>,
    instruction: SerializableInstruction,
) -> Result<()> {
    // Get accounts
    let thread = &mut ctx.accounts.thread;

    // Append the instruction.
    thread.instructions.push(instruction);

    // TODO Realloc account space.

    Ok(())
}
