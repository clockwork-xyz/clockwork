use {
    crate::objects::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(pool_authority: Pubkey)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        address = Config::pubkey(),
        payer = admin,
        space = 8 + size_of::<Config>(),
    )]
    pub config: Account<'info, Config>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, pool_authority: Pubkey) -> Result<()> {
    let admin = &ctx.accounts.admin;
    let config = &mut ctx.accounts.config;

    config.init(admin.key(), pool_authority)?;

    Ok(())
}
