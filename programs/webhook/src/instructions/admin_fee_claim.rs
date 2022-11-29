use {
    crate::state::{Config, Fee, FeeAccount, SEED_FEE},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct AdminFeeClaim<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(address = Config::pubkey(), has_one = admin)]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub pay_to: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [
            SEED_FEE,
            fee.authority.key().as_ref()
        ],
        bump,
    )]
    pub fee: Account<'info, Fee>,
}

pub fn handler<'info>(ctx: Context<AdminFeeClaim>, amount: u64) -> Result<()> {
    // Get accounts
    let pay_to = &mut ctx.accounts.pay_to;
    let fee = &mut ctx.accounts.fee;

    // Claim the fee funds
    fee.claim_admin_balance(amount, pay_to)?;

    Ok(())
}
