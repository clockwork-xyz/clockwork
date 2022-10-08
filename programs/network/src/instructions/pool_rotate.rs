use {crate::objects::*, anchor_lang::prelude::*};

// TODO Make pool rotation a function of the epoch pubkey.
//      Workers should self-select into the delegate pool on deterministic epochs.
//      If a worker is not active, they will not rotate into the pool. 
//      This gives curent workers (presumably active) extra time in the pool.

#[derive(Accounts)]
pub struct PoolRotate<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub delegate: Signer<'info>,

    #[account(
        address = snapshot_frame.pubkey(),
        has_one = snapshot,
        has_one = worker,
        constraint = is_valid_frame(&rotator, &snapshot, &snapshot_frame).unwrap()
    )]
    pub snapshot_frame: Account<'info, SnapshotFrame>,

    #[account(
        mut,
        seeds = [SEED_ROTATOR], 
        bump,
        constraint = Clock::get().unwrap().slot >= rotator.last_rotation_at.checked_add(config.slots_per_rotation).unwrap()
    )]
    pub rotator: Account<'info, Rotator>,

    #[account(
        address = snapshot.pubkey(),
        
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(address = worker.pubkey())]
    pub worker: Account<'info, Worker>,
}

pub fn handler(_ctx: Context<PoolRotate>) -> Result<()> {
    // Get accounts
    // let rotator = &mut ctx.accounts.rotator;

    // TODO If this was a valid request, rotate the worker into the pool

    // Hash the rotator's nonce value
    // TODO This should be done automatically on a 
    // rotator.hash_nonce()?;

    Ok(())
}

fn is_valid_frame(
    rotator: &Account<Rotator>,
    snapshot: &Account<Snapshot>,
    snapshot_frame: &Account<SnapshotFrame>,
) -> Result<bool> {
    // Return true if the sample is within the entry's stake range
    match rotator.nonce.checked_rem(snapshot.total_stake) {
        None => Ok(false),
        Some(sample) => Ok(sample >= snapshot_frame.stake_offset
            && sample < snapshot_frame.stake_offset.checked_add(snapshot_frame.stake_amount).unwrap()),
    }
}
