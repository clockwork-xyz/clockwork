use {crate::state::*, anchor_lang::prelude::*};

/// Accounts required by the `thread_delete` instruction.
#[derive(Accounts)]
pub struct ThreadDelete<'info> {
    /// The authority (owner) of the thread.
    #[account(
        constraint = authority.key().eq(&thread.authority) || authority.key().eq(&thread.key())
    )]
    pub authority: Signer<'info>,

    /// The address to return the data rent lamports to.
    #[account(mut)]
    pub close_to: SystemAccount<'info>,

    /// The thread to be delete.
    #[account(
        mut,
        seeds = [
            SEED_THREAD,
            thread.authority.as_ref(),
            thread.id.as_slice(),
        ],
        bump = thread.bump,
    )]
    pub thread: Account<'info, Thread>,
}

pub fn handler(ctx: Context<ThreadDelete>) -> Result<()> {
    let thread = &ctx.accounts.thread;
    let close_to = &ctx.accounts.close_to;

    let thread_lamports = thread.to_account_info().lamports();
    **thread.to_account_info().try_borrow_mut_lamports()? = thread
        .to_account_info()
        .lamports()
        .checked_sub(thread_lamports)
        .unwrap();
    **close_to.to_account_info().try_borrow_mut_lamports()? = close_to
        .to_account_info()
        .lamports()
        .checked_add(thread_lamports)
        .unwrap();

    Ok(())
}
