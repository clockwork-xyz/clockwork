use std::mem::size_of;

use anchor_lang::{
    prelude::*,
    solana_program::system_program,
};

use crate::state::*;


/// Accounts required by the `thread_create` instruction.
#[derive(Accounts)]
#[instruction(address_lookup_tables: Vec<Pubkey>)]
pub struct LookupTablesCreate<'info> {
    /// The authority (owner) of the thread.
    #[account()]
    pub authority: Signer<'info>,

    /// The payer for account initializations. 
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The Solana system program.
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

    /// The thread to be created.
    #[account(
        init,
        seeds = [
            SEED_LOOKUP,
            authority.key().as_ref(),
            thread.key().as_ref(),
        ],
        bump,
        payer= payer,
        space = vec![
            8, 
            size_of::<LookupTables>(), 
            address_lookup_tables.try_to_vec()?.len(),  
        ].iter().sum()
    )]
    pub lookup_tables: Account<'info, LookupTables>,
}

pub fn handler(ctx: Context<LookupTablesCreate>, address_lookup_tables: Vec<Pubkey>) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let thread = &ctx.accounts.thread;
    let lookup_tables = &mut ctx.accounts.lookup_tables;

    // Initialize the thread
    let bump = *ctx.bumps.get("lookup_tables").unwrap();
    lookup_tables.authority = authority.key();
    lookup_tables.bump = bump;
    lookup_tables.thread = thread.key();
    lookup_tables.lookup_tables = address_lookup_tables;

    Ok(())
}
