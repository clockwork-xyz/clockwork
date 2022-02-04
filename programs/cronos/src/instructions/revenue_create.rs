use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct RevenueCreate<'info> {
    #[account(
        seeds = [
            SEED_DAEMON, 
            daemon.owner.as_ref()
        ],
        bump = daemon.bump,
        owner = crate::ID
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        init,
        seeds = [
            SEED_REVENUE, 
            daemon.key().as_ref()
        ],
        bump = bump,
        payer = signer,
        space = 8 + size_of::<Revenue>(),
    )]
    pub revenue: Account<'info, Revenue>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<RevenueCreate>, bump: u8) -> ProgramResult {
    // Get accounts.
    let daemon = &mut ctx.accounts.daemon;
    let revenue = &mut ctx.accounts.revenue;

    // Initialize revenue account.
    revenue.daemon = daemon.key();
    revenue.balance = 0;
    revenue.bump = bump;

    Ok(())
}
