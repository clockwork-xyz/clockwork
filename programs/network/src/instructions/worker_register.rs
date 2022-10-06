use {
    crate::objects::*,
    anchor_lang::{
        prelude::*,
        solana_program::{system_program},
    },
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct WorkerRegister<'info> {
    #[account(mut, constraint = authority.key() != worker.key())]
    pub authority: Signer<'info>,

    #[account(address = Config::pubkey())]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT_ENTRY,
            snapshot.key().as_ref(),
            snapshot.total_workers.to_be_bytes().as_ref(),
        ],
        bump,
        payer = authority,
        space = 8 + size_of::<SnapshotEntry>(),
    )]
    pub entry: Account<'info, SnapshotEntry>,

    #[account(
        mut, 
        seeds = [SEED_REGISTRY],
        bump,
        constraint = !registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account()]
    pub signatory: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            snapshot.epoch.as_ref(),
        ],
        bump,
        constraint = snapshot.total_workers == registry.total_workers
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(
        init,
        seeds = [
            SEED_WORKER,
            registry.total_workers.to_be_bytes().as_ref(),
        ],
        bump,
        payer = authority,
        space = 8 + size_of::<Worker>(),
    )]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<WorkerRegister>) -> Result<()> {
    // Get accounts
    let authority = &mut ctx.accounts.authority;
    let registry = &mut ctx.accounts.registry;
    let signatory = &mut ctx.accounts.signatory;
    let worker = &mut ctx.accounts.worker;

    // Initialize the worker account.
    worker.init(authority, registry.total_workers, signatory)?;

    // Update the registry.
    registry.total_workers = registry.total_workers.checked_add(1).unwrap();

    Ok(())
}
