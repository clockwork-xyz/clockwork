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
    health_bump: u8,
    treasury_bump: u8,
)]
pub struct AdminInitialize<'info> {
    #[account(
        init,
        seeds = [SEED_AUTHORITY],
        bump = authority_bump,
        payer = signer,
        space = 8 + size_of::<Authority>(),
    )]
    pub authority: Account<'info, Authority>,

    #[account(
        init,
        seeds = [SEED_CONFIG],
        bump = config_bump,
        payer = signer,
        space = 8 + size_of::<Config>(),
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        seeds = [
            SEED_DAEMON, 
            authority.key().as_ref()
        ],
        bump = daemon_bump,
        payer = signer,
        space = 8 + size_of::<Daemon>(),
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        init,
        seeds = [
            SEED_FEE, 
            daemon.key().as_ref()
        ],
        bump = fee_bump,
        payer = signer,
        space = 8 + size_of::<Fee>(),
    )]
    pub fee: Account<'info, Fee>,

    #[account(
        init,
        seeds = [SEED_HEALTH],
        bump = health_bump,
        payer = signer,
        space = 8 + size_of::<Health>(),
    )]
    pub health: Account<'info, Health>,


    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(
        init,
        seeds = [SEED_TREASURY],
        bump = treasury_bump,
        payer = signer,
        space = 8 + size_of::<Treasury>(),
    )]
    pub treasury: Account<'info, Treasury>,
}

pub fn handler(
    ctx: Context<AdminInitialize>,
    authority_bump: u8,
    config_bump: u8,
    daemon_bump: u8,
    fee_bump: u8,
    health_bump: u8,
    treasury_bump: u8,
) -> ProgramResult {
    let authority = &mut ctx.accounts.authority;
    let config = &mut ctx.accounts.config;
    let daemon = &mut ctx.accounts.daemon;
    let fee = &mut ctx.accounts.fee;
    let health = &mut ctx.accounts.health;
    let signer = &ctx.accounts.signer;
    let treasury = &mut ctx.accounts.treasury;

    authority.init(authority_bump)?;
    config.init(signer.key(), config_bump)?;
    daemon.init(authority.key(), daemon_bump)?;
    health.init(health_bump)?;
    fee.init(daemon.key(), fee_bump)?;
    treasury.init(treasury_bump)
}
