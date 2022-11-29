use {
    crate::state::{Fee, FeeAccount},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct FeeClaim<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub pay_to: SystemAccount<'info>,

    #[account(
        mut,
        address = fee.pubkey(),
        has_one = authority,
    )]
    pub fee: Account<'info, Fee>,
}

pub fn handler<'info>(ctx: Context<FeeClaim>, amount: u64) -> Result<()> {
    // Get accounts
    let pay_to = &mut ctx.accounts.pay_to;
    let fee = &mut ctx.accounts.fee;

    // Claim the fee funds
    fee.claim_worker_balance(amount, pay_to)?;

    Ok(())
}
