use anchor_lang::{
    prelude::*,
    solana_program::system_program,
    system_program::{transfer, Transfer},
};

use crate::state::*;

/// Accounts required by the `thread_instruction_add` instruction.
#[derive(Accounts)]
#[instruction(instruction: SerializableInstruction)]
pub struct ThreadInstructionAdd<'info> {
    /// The authority (owner) of the thread.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The Solana system program
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

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
    let authority = &ctx.accounts.authority;
    let thread = &mut ctx.accounts.thread;
    let system_program = &ctx.accounts.system_program;

    // Append the instruction.
    thread.instructions.push(instruction);

    // Reallocate mem for the thread account.
    thread.realloc()?;

    // If lamports are required to maintain rent-exemption, pay them.
    let data_len = thread.to_account_info().data_len();
    let minimum_rent = Rent::get().unwrap().minimum_balance(data_len);
    if minimum_rent > thread.to_account_info().lamports() {
        transfer(
            CpiContext::new(
                system_program.to_account_info(),
                Transfer {
                    from: authority.to_account_info(),
                    to: thread.to_account_info(),
                },
            ),
            minimum_rent
                .checked_sub(thread.to_account_info().lamports())
                .unwrap(),
        )?;
    }

    Ok(())
}
