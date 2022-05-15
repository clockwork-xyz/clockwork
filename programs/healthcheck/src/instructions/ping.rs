use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::sysvar},
};

#[derive(Accounts)]
pub struct Ping<'info> {
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(mut, seeds = [SEED_HEALTH], bump)]
    pub health: Account<'info, Health>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

pub fn handler(ctx: Context<Ping>) -> Result<()> {
    let clock = &ctx.accounts.clock;
    let health = &mut ctx.accounts.health;

    health.ping(clock)
}
