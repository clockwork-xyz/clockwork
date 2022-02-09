use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::sysvar},
};

#[derive(Accounts)]
#[instruction()]
pub struct AdminResetHealth<'info> {
    #[account(mut, address = config.admin)]
    pub admin: Signer<'info>,

    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        owner = crate::ID,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_HEALTH],
        bump = health.bump,
        owner = crate::ID,
    )]
    pub health: Account<'info, Health>,
}

pub fn handler(ctx: Context<AdminResetHealth>) -> ProgramResult {
    // Get accounts.
    let clock = &ctx.accounts.clock;
    let health = &mut ctx.accounts.health;

    // Update the health account.
    health.target_time = clock.unix_timestamp;
    health.real_time = clock.unix_timestamp;

    Ok(())
}
