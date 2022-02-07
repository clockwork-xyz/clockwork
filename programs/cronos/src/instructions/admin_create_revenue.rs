use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct AdminCreateRevenue<'info> {
    #[account(mut, address = config.admin)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [SEED_AUTHORITY], 
        bump = authority.bump, 
        owner = crate::ID
    )]
    pub authority: Account<'info, Authority>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        owner = crate::ID,
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [
            SEED_DAEMON, 
            daemon.owner.as_ref()
        ],
        bump = daemon.bump,
        constraint = daemon.owner == authority.key(),
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        init,
        seeds = [
            SEED_REVENUE, 
            daemon.key().as_ref()
        ],
        bump = bump,
        payer = admin,
        space = 8 + size_of::<Revenue>(),
    )]
    pub revenue: Account<'info, Revenue>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AdminCreateRevenue>, bump: u8) -> ProgramResult {
     // Get accounts.
     let daemon = &mut ctx.accounts.daemon;
     let revenue = &mut ctx.accounts.revenue;
 
     // Initialize revenue account.
     revenue.daemon = daemon.key();
     revenue.balance = 0;
     revenue.bump = bump;

    Ok(())
}
