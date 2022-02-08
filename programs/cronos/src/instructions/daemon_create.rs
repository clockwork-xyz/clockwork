use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(
    daemon_bump: u8,
    fee_bump: u8,
)]
pub struct DaemonCreate<'info> {
    #[account(
        init,
        seeds = [
            SEED_DAEMON, 
            owner.key().as_ref()
        ],
        bump = daemon_bump,
        payer = owner,
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
        payer = owner,
        space = 8 + size_of::<Fee>(),
    )]
    pub fee: Account<'info, Fee>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DaemonCreate>, daemon_bump: u8, fee_bump: u8) -> ProgramResult {
    // Get accounts.
    let daemon = &mut ctx.accounts.daemon;
    let fee = &mut ctx.accounts.fee;
    let owner = &ctx.accounts.owner;

    // Initialize daemon account.
    daemon.owner = owner.key();
    daemon.task_count = 0;
    daemon.bump = daemon_bump;

    // Initialize fee account.
    fee.daemon = daemon.key();
    fee.balance = 0;
    fee.bump = fee_bump;

    Ok(())
}
