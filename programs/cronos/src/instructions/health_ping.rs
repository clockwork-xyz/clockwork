use crate::state::*;

use anchor_lang::prelude::*; 
use solana_program::sysvar;

#[derive(Accounts)]
#[instruction()]
pub struct HealthPing<'info> {
    #[account(
        seeds = [SEED_AUTHORITY], 
        bump = authority.bump, 
    )]
    pub authority: Account<'info, Authority>,
    
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
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
        signer 
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        mut,
        seeds = [SEED_HEALTH],
        bump = health.bump,
    )]
    pub health: Account<'info, Health>,
}

pub fn handler(ctx: Context<HealthPing>) -> Result<()> {
    let clock = &ctx.accounts.clock;
    let config = &ctx.accounts.config;
    let health = &mut ctx.accounts.health;

    health.ping(clock, config)
}
