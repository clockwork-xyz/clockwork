use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    anchor_spl::token::Mint,
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [SEED_CONFIG],
        bump,
        payer = admin,
        space = 8 + size_of::<Config>(),
    )]
    pub config: Account<'info, Config>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        seeds = [SEED_REGISTRY],
        bump,
        payer = admin,
        space = 8 + size_of::<Registry>(),
    )]
    pub registry: Account<'info, Registry>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT,
            (0 as u64).to_be_bytes().as_ref(),
        ],
        bump,
        payer = admin,
        space = 8 + size_of::<Snapshot>(),
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    // Get accounts
    let admin = &ctx.accounts.admin;
    let config = &mut ctx.accounts.config;
    let mint = &ctx.accounts.mint;
    let registry = &mut ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;

    // Initialize accounts.
    config.init(admin.key(), mint.key())?;
    registry.init()?;
    snapshot.init(0)?;

    Ok(())
}
