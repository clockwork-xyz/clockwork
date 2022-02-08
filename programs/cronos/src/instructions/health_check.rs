use {
    crate::state::*,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction()]
pub struct HealthCheck<'info> {
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [SEED_AUTHORITY], 
        bump = authority.bump, 
        owner = crate::ID,
    )]
    pub authority: Account<'info, Authority>,

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
    let health = &mut ctx.accounts.health;

    // Update the health account.
    health.target_time = health.target_time.checked_add(1).unwrap();
    health.real_time = clock.unix_timestamp;

    Ok(())
}
