use {crate::{errors::*, objects::*}, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct PoolRotate<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = entry.pubkey(),
        has_one = snapshot,
        has_one = worker,
        constraint = is_valid_entry(&entry, &rotator, &snapshot).unwrap()
    )]
    pub entry: Account<'info, SnapshotEntry>,

    #[account(
        mut,
        seeds = [SEED_ROTATOR], 
        bump,
        constraint = Clock::get().unwrap().slot >= rotator.last_rotation_at.checked_add(config.slots_per_rotation).unwrap()
    )]
    pub rotator: Account<'info, Rotator>,

    #[account(
        address = snapshot.pubkey(),
        constraint = snapshot.status == SnapshotStatus::Current
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(address = worker.pubkey())]
    pub worker: Account<'info, Node>,
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

fn is_valid_entry(
    entry: &Account<SnapshotEntry>,
    rotator: &Account<Rotator>,
    snapshot: &Account<Snapshot>,
) -> Result<bool> {
    // Return true if the sample is within the entry's stake range
    match rotator.nonce.checked_rem(snapshot.stake_total) {
        None => Ok(false),
        Some(sample) => Ok(sample >= entry.stake_offset
            && sample < entry.stake_offset.checked_add(entry.stake_amount).unwrap()),
    }
}
