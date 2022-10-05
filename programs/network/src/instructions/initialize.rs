use {
    crate::objects::*,
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

    #[account(
        init,
        seeds = [SEED_EPOCH, (0 as u64).to_be_bytes().as_ref()],
        bump,
        payer = admin,
        space = 8 + size_of::<Epoch>(),
    )]
    pub epoch: Account<'info, Epoch>,

    #[account(
        init,
        seeds = [SEED_ROTATOR],
        bump,
        payer = admin,
        space = 8 + size_of::<Rotator>(),
    )]
    pub rotator: Account<'info, Rotator>,

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

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
    // Get accounts
    let admin = &ctx.accounts.admin;
    let config = &mut ctx.accounts.config;
    let epoch = &ctx.accounts.epoch;
    let rotator = &mut ctx.accounts.rotator;
    let mint = &ctx.accounts.mint;
    let registry = &mut ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;

    // Initialize accounts.
    config.init(admin.key(), mint.key())?;
    registry.init()?;
    rotator.init()?;

    // TODO Create the 0th epoch.

    // Take the first snapshot.
    // registry.new_snapshot(snapshot)?;
    // registry.rotate_snapshot(None, snapshot)?;

    Ok(())
}
