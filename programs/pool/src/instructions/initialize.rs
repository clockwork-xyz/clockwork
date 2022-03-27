use {
    crate::state::*,
    anchor_lang::prelude::*,
    solana_program::system_program,
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(
    config_bump: u8,
)]
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

pub fn handler(
    ctx: Context<Initialize>,
    config_bump: u8,
) -> Result<()> {
    let admin = &ctx.accounts.admin;
    let config = &mut ctx.accounts.config;

    config.new(admin.key(), config_bump)?;

    Ok(())
}
