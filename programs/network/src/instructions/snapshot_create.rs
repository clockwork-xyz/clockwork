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

    #[account(address = new_epoch.pubkey())]
    pub new_epoch: Account<'info, Epoch>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = config.authorized_queue)]
    pub queue: Signer<'info>,

    #[account(
        mut, 
        seeds = [SEED_REGISTRY], 
        bump,
        constraint = !registry.is_locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT,
            new_epoch.key().as_ref(),
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
    let new_epoch = &ctx.accounts.new_epoch;
    let registry = &mut ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;

    // Start a new snapshot.
    snapshot.init(new_epoch.key())?;

    Ok(())
}
