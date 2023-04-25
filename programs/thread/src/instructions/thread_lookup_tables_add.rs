use anchor_lang::{
    prelude::*,
    solana_program::system_program,
    system_program::{transfer, Transfer},
};

use crate::state::*;

/// Accounts required by the `thread_address_lookup_tables_add` instruction.
#[derive(Accounts)]
#[instruction(address_lookup_tables: Vec<Pubkey>)]
pub struct LookupTablesAdd<'info> {
    /// The authority (owner) of the thread.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The Solana system program
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    /// The thread to be paused.
    #[account(
        seeds = [
            SEED_THREAD,
            thread.authority.as_ref(),
            thread.id.as_slice(),
        ],
        bump = thread.bump,
        has_one = authority
    )]
    pub thread: Account<'info, Thread>,

    /// The lookup tables to be edited.
    #[account(
        mut,
        seeds = [
            SEED_LOOKUP,
            lookup_tables.authority.as_ref(),
            lookup_tables.thread.as_ref(),
        ],
        bump = lookup_tables.bump,
        has_one = authority,
        has_one = thread,
    )]
    pub lookup_tables: Account<'info, LookupTables>,
}

pub fn handler(
    ctx: Context<LookupTablesAdd>,
    address_lookup_tables: Vec<Pubkey>,
) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let lookup_tables = &mut ctx.accounts.lookup_tables;
    let system_program = &ctx.accounts.system_program;

    // Append the address lookup tables.
    lookup_tables.lookup_tables.extend(address_lookup_tables.iter());

    // Reallocate mem for the thread account.
    lookup_tables.realloc()?;

    // If lamports are required to maintain rent-exemption, pay them.
    let data_len = 8 + lookup_tables.try_to_vec()?.len();
    let minimum_rent = Rent::get().unwrap().minimum_balance(data_len);
    if minimum_rent > lookup_tables.to_account_info().lamports() {
        transfer(
            CpiContext::new(
                system_program.to_account_info(),
                Transfer {
                    from: authority.to_account_info(),
                    to: lookup_tables.to_account_info(),
                },
            ),
            minimum_rent
                .checked_sub(lookup_tables.to_account_info().lamports())
                .unwrap(),
        )?;
    }

    Ok(())
}
