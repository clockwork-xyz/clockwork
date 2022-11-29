use {
    crate::state::{Config, ConfigAccount, SEED_CONFIG},
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [SEED_CONFIG],
        bump,
        payer = admin,
        space = 8 + size_of::<Config>(),
    )]
    pub config: Account<'info, Config>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(ctx: Context<Initialize>) -> Result<()> {
    // Get accounts
    let admin = &ctx.accounts.admin;
    let config = &mut ctx.accounts.config;

    // Initialize the config account
    config.init(admin.key())?;

    Ok(())
}
