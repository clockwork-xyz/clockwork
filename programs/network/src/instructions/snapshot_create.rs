use {
    crate::objects::*,
    anchor_lang::prelude::*,
    std::mem::size_of,
};

// TODO Permission this function to a config pubkey

#[derive(Accounts)]
pub struct SnapshotCreate<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut, 
        seeds = [SEED_REGISTRY], 
        bump,
        constraint = !registry.is_locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = config.automation_authority)]
    pub signer: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT,
            registry.snapshot_count.to_be_bytes().as_ref(),
        ],
        bump,
        space = 8 + size_of::<Snapshot>(),
        payer = payer
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account()]
    pub system_program: Program<'info, System>,
}

// TODO Add condition to check if enough time has passed since the last snapshot.

pub fn handler(ctx: Context<SnapshotCreate>) -> Result<()> {
    // Get accounts
    let registry = &mut ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;

    // Start a new snapshot
    registry.new_snapshot(snapshot)?;

    Ok(())
}
