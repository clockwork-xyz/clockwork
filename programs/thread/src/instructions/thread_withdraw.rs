use {crate::objects::*, anchor_lang::prelude::*};

/// Accounts required by the `thread_withdraw` instruction.
#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct ThreadWithdraw<'info> {
    /// The authority (owner) of the thread.
    #[account()]
    pub authority: Signer<'info>,

    /// The account to withdraw lamports to.
    #[account(mut)]
    pub pay_to: SystemAccount<'info>,

    /// The thread to be.
    #[account(
        mut,
        seeds = [
            SEED_THREAD,
            thread.authority.as_ref(),
            thread.id.as_bytes(),
        ],
        bump,
        has_one = authority,
    )]
    pub thread: Account<'info, Thread>,
}

pub fn handler(ctx: Context<ThreadWithdraw>, amount: u64) -> Result<()> {
    // Get accounts
    let pay_to = &mut ctx.accounts.pay_to;
    let thread = &mut ctx.accounts.thread;

    // Withdraw balance from thread to the pay_to account
    **thread.to_account_info().try_borrow_mut_lamports()? = thread
        .to_account_info()
        .lamports()
        .checked_sub(amount)
        .unwrap();
    **pay_to.to_account_info().try_borrow_mut_lamports()? = pay_to
        .to_account_info()
        .lamports()
        .checked_add(amount)
        .unwrap();

    Ok(())
}
