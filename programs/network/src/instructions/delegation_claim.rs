use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct DelegationClaim<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub pay_to: SystemAccount<'info>,

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

pub fn handler(ctx: Context<DelegationClaim>, amount: u64) -> Result<()> {
    // Get accounts.
    let pay_to = &mut ctx.accounts.pay_to;
    let delegation = &mut ctx.accounts.delegation;

    // Decrement the delegation's claimable balance.
    delegation.yield_balance = delegation.yield_balance.checked_sub(amount).unwrap();

    // Transfer commission to the worker.
    **delegation.to_account_info().try_borrow_mut_lamports()? = delegation
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
