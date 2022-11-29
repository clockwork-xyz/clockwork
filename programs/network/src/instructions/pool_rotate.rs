use {
    crate::{errors::*, state::*},
    anchor_lang::prelude::*,
};

// TODO Make pool rotation a function of the epoch pubkey.
//      Workers should self-select into the delegate pool on deterministic epochs.
//      If a worker is not active, they will not rotate into the pool.
//      This gives curent workers (presumably active) extra time in the pool.

#[derive(Accounts)]
pub struct PoolRotate<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_POOL,
            pool.id.to_be_bytes().as_ref(),
        ],
        bump,
    )]
    pub pool: Account<'info, Pool>,

    #[account(address = Registry::pubkey())]
    pub registry: Account<'info, Registry>,

    #[account(mut)]
    pub signatory: Signer<'info>,

    #[account(
        address = snapshot.pubkey(),
        constraint = snapshot.id.eq(&registry.current_epoch)
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(
        address = snapshot_frame.pubkey(),
        has_one = snapshot,
        has_one = worker
    )]
    pub snapshot_frame: Account<'info, SnapshotFrame>,

    #[account(
        address = worker.pubkey(),
        has_one = signatory
    )]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<PoolRotate>) -> Result<()> {
    // Get accounts
    let pool = &mut ctx.accounts.pool;
    let registry = &ctx.accounts.registry;
    let snapshot = &ctx.accounts.snapshot;
    let snapshot_frame = &ctx.accounts.snapshot_frame;
    let worker = &ctx.accounts.worker;

    // Verify the pool has excess space or the worker can rotate in at this time.
    require!(
        pool.workers.len().lt(&pool.size)
            || is_rotation_window_open(&registry, &snapshot, &snapshot_frame).unwrap(),
        ClockworkError::PoolFull
    );

    // Verify the worker is not already in the pool.
    require!(
        !pool.workers.contains(&worker.key()),
        ClockworkError::AlreadyInPool
    );

    // Rotate the worker into the pool.
    pool.rotate(worker.key())?;

    Ok(())
}

fn is_rotation_window_open(
    registry: &Account<Registry>,
    snapshot: &Account<Snapshot>,
    snapshot_frame: &Account<SnapshotFrame>,
) -> Result<bool> {
    // Return true if the sample is within the entry's stake range
    match registry.nonce.checked_rem(snapshot.total_stake) {
        None => Ok(false),
        Some(sample) => Ok(sample >= snapshot_frame.stake_offset
            && sample
                < snapshot_frame
                    .stake_offset
                    .checked_add(snapshot_frame.stake_amount)
                    .unwrap()),
    }
}
