use {
    crate::state::*,
    anchor_lang::prelude::*,
    solana_program::sysvar ,
};

#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(
        mut, 
        address = config.admin
    )]
    pub admin: Signer<'info>,

    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_HEARTBEAT],
        bump = heartbeat.bump,
    )]
    pub heartbeat: Account<'info, Heartbeat>,
}

pub fn handler(ctx: Context<Reset>) -> Result<()> {
    let clock = &ctx.accounts.clock;
    let heartbeat = &mut ctx.accounts.heartbeat;

    heartbeat.reset(clock)
}
