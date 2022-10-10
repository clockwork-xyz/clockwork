use {crate::objects::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct DelegationYield<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_DELEGATION,
            delegation.worker.as_ref(),
            delegation.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = authority,
    )]
    pub delegation: Account<'info, Delegation>,
}

pub fn handler(ctx: Context<DelegationYield>, amount: u64) -> Result<()> {
    // Get accounts.
    let authority = &mut ctx.accounts.authority;
    let delegation = &mut ctx.accounts.delegation;

    // Decrement the delegation's claimable balance.
    delegation.claimable_balance = delegation.claimable_balance.checked_sub(amount).unwrap();

    // Transfer commission to the worker.
    **delegation.to_account_info().try_borrow_mut_lamports()? = delegation
        .to_account_info()
        .lamports()
        .checked_sub(amount)
        .unwrap();
    **authority.to_account_info().try_borrow_mut_lamports()? = authority
        .to_account_info()
        .lamports()
        .checked_add(amount)
        .unwrap();

    Ok(())
}
