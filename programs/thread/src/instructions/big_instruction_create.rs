use std::mem::size_of;

use anchor_lang::{
    prelude::*,
    solana_program::system_program,
};

use crate::{errors::ClockworkError, state::*};

/// Accounts required by the `thread_lookup_tables_create` instruction.
#[derive(Accounts)]
#[instruction(id: Vec<u8>, instruction_data: Vec<u8>, no_of_accounts: u8)]
pub struct BigInstructionCreate<'info> {
    /// The authority (owner) of the thread.
    #[account()]
    pub authority: Signer<'info>,

    /// The payer for account initializations. 
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The Solana system program.
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    /// CHECK: program_id of the instruction to build
    #[account(executable)]
    pub instruction_program_id: UncheckedAccount<'info>,

    /// The lookup_tables account to be created.
    #[account(
        init,
        seeds = [
            SEED_BIG_INST,
            authority.key().as_ref(),
            instruction_program_id.key().as_ref(),
            id.as_slice(),
        ],
        bump,
        payer= payer,
        space = vec![
            8, 
            size_of::<BigInstruction>(),
            id.len(),
            instruction_data.try_to_vec()?.len(),
            (no_of_accounts as usize) * size_of::<AccountMeta>(),
        ].iter().sum()
    )]
    pub big_instruction: Account<'info, BigInstruction>,
}

pub fn handler(ctx: Context<BigInstructionCreate>, id: Vec<u8>, instruction_data: Vec<u8>, no_of_accounts: u8) -> Result<()> {
    // Get accounts
    require!((no_of_accounts as usize) == ctx.remaining_accounts.len(), ClockworkError::AccountsLengthMismatch);

    let authority = &ctx.accounts.authority;
    let program_id = &ctx.accounts.instruction_program_id;
    let big_instruction = &mut ctx.accounts.big_instruction;
    let accounts = ctx.remaining_accounts.into_iter().map(|acct_info| SerializableAccount {
        is_signer: acct_info.is_signer, // false
        is_writable: acct_info.is_writable, 
        pubkey: acct_info.key()
    }).collect::<Vec<SerializableAccount>>();

    // Initialize the thread
    let bump = *ctx.bumps.get("big_instruction").unwrap();
    big_instruction.authority = authority.key();
    big_instruction.bump = bump;
    big_instruction.id = id;
    big_instruction.data = instruction_data;
    big_instruction.program_id = program_id.key();
    big_instruction.accounts = accounts;

    Ok(())
}
