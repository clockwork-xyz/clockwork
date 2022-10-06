use {crate::{errors::*, objects::*}, anchor_lang::prelude::*};

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

pub fn handler(ctx: Context<PoolRotate>) -> Result<()> {
    // Get accounts
    let rotator = &mut ctx.accounts.rotator;

    // Rotate the worker into its supported pools
    for i in 0..ctx.remaining_accounts.len() {
        match ctx.remaining_accounts.get(i) {
            None => return Err(ClockworkError::InvalidPool.into()),
            Some(pool_acc_info) => {

                // Verify the account pubkey is an expected pool
                require!(pool_acc_info.key().eq(rotator.pool_pubkeys.get(i).unwrap()), ClockworkError::InvalidPool);

                // If the node supports this pool, then rotate it in
                // if node.supported_pools.contains(&pool_acc_info.key()) {
                    // clockwork_pool_program::cpi::pool_rotate(
                    //     CpiContext::new_with_signer(
                    //         pool_program.to_account_info(),
                    //         PoolRotate {
                    //             config: pool_program_config.to_account_info(),
                    //             pool: pool_acc_info.clone(),
                    //             pool_authority: rotator.to_account_info(),
                    //             worker: worker.to_account_info(),
                    //         },
                    //         &[&[SEED_ROTATOR, &[bump]]],
                    //     ),
                    // )?;
                    // pool.rotate(worker.key())?;
                // }
            }
        }
    }

    // Hash the rotator's nonce value
    rotator.hash_nonce()?;

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
