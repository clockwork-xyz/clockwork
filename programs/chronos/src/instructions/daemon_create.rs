use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct DaemonCreate<'info> {
    #[account(
        init,
        seeds = [SEED_DAEMON, signer.key().as_ref()],
        bump = bump,
        payer = signer,
        space = 8 + size_of::<Daemon>(),
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DaemonCreate>, bump: u8) -> ProgramResult {
    // Get accounts.
    let daemon = &mut ctx.accounts.daemon;
    let signer = &ctx.accounts.signer;

    // Initialize daemon account.
    daemon.owner = signer.key();
    daemon.bump = bump;

    return Ok(());
}
