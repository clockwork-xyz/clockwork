use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [SEED_AUTHORITY],
        bump,
        payer = admin,
        space = 8 + size_of::<Authority>(),
    )]
    pub authority: Account<'info, Authority>,

    #[account(
        init,
        seeds = [SEED_CONFIG],
        bump,
        payer = admin,
        space = 8 + size_of::<Config>(),
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        seeds = [
            SEED_FEE, 
            yogi.key().as_ref()
        ],
        bump,
        payer = admin,
        space = 8 + size_of::<Fee>(),
    )]
    pub fee: Account<'info, Fee>,

    // TODO Pool

    #[account(
        init,
        seeds = [
            SEED_YOGI,
            authority.key().as_ref()
        ],
        bump,
        payer = admin,
        space = 8 + size_of::<Yogi>(),
    )]
    pub yogi: Account<'info, Yogi>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let admin = &ctx.accounts.admin;
    let authority = &mut ctx.accounts.authority;
    let config = &mut ctx.accounts.config;
    let yogi = &mut ctx.accounts.yogi;
    let fee = &mut ctx.accounts.fee;

    let yogi_bump = *ctx.bumps.get("yogi").unwrap();

    config.new(admin.key(), admin.key())?; // TODO pool_pubkey = pool.key()
    yogi.new(yogi_bump, authority.key())?;
    fee.new(yogi.key())?;

    Ok(())
}
