use {crate::state::*, anchor_lang::prelude::*};

/// Accounts required by the `thread_instruction_remove` instruction.
#[derive(Accounts)]
#[instruction(index: u64)]
pub struct LookupTablesRemove<'info> {
    /// The authority (owner) of the thread.
    #[account()]
    pub authority: Signer<'info>,

    // the thread owner of the lookup tables
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

pub fn handler(ctx: Context<LookupTablesRemove>, index: u64) -> Result<()> {
    // Get accounts
    let lookup_tables = &mut ctx.accounts.lookup_tables;

    // remove the lookup table key
    lookup_tables.lookup_tables.remove(index as usize);

    Ok(())
}
