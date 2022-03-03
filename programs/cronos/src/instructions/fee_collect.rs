use {
    crate::state::*,
    anchor_lang::prelude::*
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

pub fn handler(ctx: Context<FeeCollect>) -> Result<()> {
    let fee = &mut ctx.accounts.fee;
    let treasury = &mut ctx.accounts.treasury;

    fee.collect(treasury)
}
