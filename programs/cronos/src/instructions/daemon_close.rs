use {
    crate::state::*,
    anchor_lang::prelude::*
};

#[derive(Accounts)]
pub struct DaemonClose<'info> {
    #[account(
        mut,
        seeds = [
            SEED_DAEMON, 
            daemon.owner.as_ref()
        ],
        bump = daemon.bump,
        has_one = owner,
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn handler(ctx: Context<DaemonClose>) -> Result<()> {
    let daemon = &mut ctx.accounts.daemon;
    let owner = &mut ctx.accounts.owner;

    daemon.close(owner)
}
