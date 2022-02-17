use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::sysvar},
};

#[derive(Accounts)]
#[instruction()]
pub struct HealthCheck<'info> {
    #[account(
        seeds = [SEED_AUTHORITY], 
        bump = authority.bump, 
        owner = crate::ID,
    )]
    pub authority: Account<'info, Authority>,
    
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        owner = crate::ID
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_DAEMON, 
            daemon.owner.as_ref()
        ],
        bump = daemon.bump,
        constraint = daemon.owner == authority.key(),
        owner = crate::ID,
        signer 
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        mut,
        seeds = [SEED_HEALTH],
        bump = health.bump,
        owner = crate::ID,
    )]
    pub health: Account<'info, Health>,
}

pub fn handler(ctx: Context<HealthCheck>) -> ProgramResult {
    // Get accounts.
    let clock = &ctx.accounts.clock;
    let config = &ctx.accounts.config;
    let health = &mut ctx.accounts.health;

    // Update the health account.
    health.last_ping = clock.unix_timestamp;
    health.target_ping = health.target_ping.checked_add(config.min_recurr).unwrap();

    Ok(())
}
