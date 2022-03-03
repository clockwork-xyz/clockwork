use {
    crate::state::*,
    anchor_lang::prelude::*
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct DaemonWidthdraw<'info> {
    #[account(
        mut,
        seeds = [
            SEED_DAEMON, 
            daemon.owner.as_ref()
        ],
        bump = daemon.bump,
        constraint = daemon.owner == owner.key()
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn handler(ctx: Context<DaemonWidthdraw>, amount: u64) -> Result<()> {
    let daemon = &mut ctx.accounts.daemon;
    let owner = &ctx.accounts.owner;

    daemon.widthdraw(amount, owner)
}
