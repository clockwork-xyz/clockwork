use {
    crate::state::*, 
    anchor_lang::{prelude::*, solana_program::sysvar}
};

#[derive(Accounts)]
#[instruction()]
pub struct HeartbeatPing<'info> {
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        mut,
        seeds = [SEED_HEARTBEAT],
        bump = heartbeat.bump,
    )]
    pub heartbeat: Account<'info, Heartbeat>,

    #[account(mut)]
    pub signer: Signer<'info>,
}

pub fn handler(ctx: Context<HeartbeatPing>) -> Result<()> {
    let clock = &ctx.accounts.clock;
    let heartbeat = &mut ctx.accounts.heartbeat;

    heartbeat.ping(clock)
}
