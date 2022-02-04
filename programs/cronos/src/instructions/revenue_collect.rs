use {
    crate::state::*,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction()]
pub struct RevenueCollect<'info> {
    #[account(
        mut,
        seeds = [
            SEED_REVENUE, 
            revenue.daemon.as_ref()
        ],
        bump = revenue.bump,
    )]
    pub revenue: Account<'info, Revenue>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_TREASURY],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,
}

pub fn handler(ctx: Context<RevenueCollect>) -> ProgramResult {
    // Get accounts.
    let revenue = &mut ctx.accounts.revenue;
    let treasury = &mut ctx.accounts.treasury;

    // Collect lamports revenue account to treasury.
    **revenue.to_account_info().try_borrow_mut_lamports()? -= revenue.balance;
    **treasury.to_account_info().try_borrow_mut_lamports()? += revenue.balance;

    // Null out collectable revenue balance.
    revenue.balance = 0;
    
    Ok(())
}
