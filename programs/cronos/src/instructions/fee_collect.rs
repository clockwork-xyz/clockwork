use {
    crate::state::*,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction()]
pub struct FeeCollect<'info> {
    #[account(
        mut,
        seeds = [
            SEED_FEE, 
            fee.daemon.as_ref()
        ],
        bump = fee.bump,
    )]
    pub fee: Account<'info, Fee>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_TREASURY],
        bump = treasury.bump,
    )]
    pub treasury: Account<'info, Treasury>,
}

pub fn handler(ctx: Context<FeeCollect>) -> ProgramResult {
    // Get accounts.
    let fee = &mut ctx.accounts.fee;
    let treasury = &mut ctx.accounts.treasury;

    // Collect lamports fee account to treasury.
    **fee.to_account_info().try_borrow_mut_lamports()? = fee.to_account_info().lamports().checked_sub(fee.balance).unwrap();
    **treasury.to_account_info().try_borrow_mut_lamports()? = treasury.to_account_info().lamports().checked_add(fee.balance).unwrap();

    // Null out collectable fee balance.
    fee.balance = 0;
    
    Ok(())
}
