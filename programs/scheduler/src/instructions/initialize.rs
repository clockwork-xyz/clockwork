use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(
    authority_bump: u8,
    config_bump: u8,
    daemon_bump: u8,
    fee_bump: u8,
)]
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
            SEED_DAEMON, 
            authority.key().as_ref()
        ],
        bump,
        payer = admin,
        space = 8 + size_of::<Daemon>(),
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        init,
        seeds = [
            SEED_FEE, 
            daemon.key().as_ref()
        ],
        bump,
        payer = admin,
        space = 8 + size_of::<Fee>(),
    )]
    pub fee: Account<'info, Fee>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<Initialize>,
    authority_bump: u8,
    config_bump: u8,
    daemon_bump: u8,
    fee_bump: u8,
    registry_pubkey: Pubkey
) -> Result<()> {
    let admin = &ctx.accounts.admin;
    let authority = &mut ctx.accounts.authority;
    let config = &mut ctx.accounts.config;
    let daemon = &mut ctx.accounts.daemon;
    let fee = &mut ctx.accounts.fee;

    authority.new(authority_bump)?;
    config.new(admin.key(), config_bump, registry_pubkey)?;
    daemon.new(authority.key(), daemon_bump)?;
    fee.new(daemon.key(), fee_bump)?;

    Ok(())
}
