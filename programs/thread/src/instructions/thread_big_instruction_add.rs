use bincode::serialize;

use anchor_lang::{
    prelude::*,
    solana_program::system_program,
    system_program::{transfer, Transfer},
};

use crate::{errors::ClockworkError, state::*};

/// Accounts required by the `thread_instruction_add` instruction.
#[derive(Accounts)]
pub struct ThreadBigInstructionAdd<'info> {
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

    #[account(
        has_one=authority
    )]
    pub big_instruction: Account<'info, BigInstruction>,
}

pub fn handler(
    ctx: Context<ThreadBigInstructionAdd>,
) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let thread = &mut ctx.accounts.thread;
    let system_program = &ctx.accounts.system_program;

    let ix_data = &ctx.accounts.big_instruction.data;
    let ix_program_id = &ctx.accounts.big_instruction.program_id;
    let ix_accounts = &ctx.accounts.big_instruction.accounts;

    let build_ix = SerializableInstruction {
        accounts: ix_accounts.clone(),
        data: ix_data.clone(),
        program_id: ix_program_id.clone(),
    };

    // Check if the instruction hit next instruction size limit
    let ix_size = serialize(&build_ix).unwrap().len();
    require!(ix_size <= NEXT_INSTRUCTION_SIZE, ClockworkError::InstructionTooLarge);

    // Append the instruction.
    thread.instructions.push(build_ix);

    // Reallocate mem for the thread account.
    thread.realloc()?;

    // If lamports are required to maintain rent-exemption, pay them.
    let data_len = 8 + thread.try_to_vec()?.len();
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
