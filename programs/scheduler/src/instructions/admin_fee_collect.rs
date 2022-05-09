use {
    crate::state::*,
    anchor_lang::prelude::*
};

#[derive(Accounts)]
pub struct AdminFeeCollect<'info> {
    #[account(
        mut,
        address = config.admin
    )]
    pub admin: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_FEE, 
            fee.queue.as_ref()
        ],
        bump = fee.bump,
    )]
    pub fee: Account<'info, Fee>,
}

pub fn handler(ctx: Context<AdminFeeCollect>) -> Result<()> {
    let admin = &mut ctx.accounts.admin;
    let fee = &mut ctx.accounts.fee;

    fee.collect(admin)
}
