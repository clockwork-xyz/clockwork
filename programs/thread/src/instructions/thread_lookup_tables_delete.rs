use {crate::state::*, anchor_lang::prelude::*};

/// Accounts required by the `thread_delete` instruction.
#[derive(Accounts)]
pub struct LookupTablesDelete<'info> {
    /// The authority (owner) of the thread.
    #[account(
        constraint = authority.key().eq(&thread.authority) || authority.key().eq(&thread.key())
    )]
    pub authority: Signer<'info>,

    /// The address to return the data rent lamports to.
    #[account(mut)]
    pub close_to: SystemAccount<'info>,

    /// The thread whose lookup tables is to be closed.
    #[account(
        seeds = [
            SEED_THREAD,
            thread.authority.as_ref(),
            thread.id.as_slice(),
        ],
        bump = thread.bump,
    )]
    pub thread: Account<'info, Thread>,

    /// The lookup tables to be deleted
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

pub fn handler(ctx: Context<LookupTablesDelete>) -> Result<()> {
    let lookup_tables = &ctx.accounts.lookup_tables;
    let close_to = &ctx.accounts.close_to;

    let lookup_tables_lamports = lookup_tables.to_account_info().lamports();
    **lookup_tables.to_account_info().try_borrow_mut_lamports()? = lookup_tables
        .to_account_info()
        .lamports()
        .checked_sub(lookup_tables_lamports)
        .unwrap();
    **close_to.to_account_info().try_borrow_mut_lamports()? = close_to
        .to_account_info()
        .lamports()
        .checked_add(lookup_tables_lamports)
        .unwrap();

    Ok(())
}
