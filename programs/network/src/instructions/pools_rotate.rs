use {
    crate::{
        errors::ClockworkError,
        state::*
    },
    anchor_lang::prelude::*,
    clockwork_pool_program::cpi::accounts::PoolRotate
};

#[derive(Accounts)]
pub struct PoolsRotate<'info> {
    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [
            SEED_SNAPSHOT_ENTRY,
            entry.snapshot.as_ref(),
            entry.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = snapshot,
        has_one = worker,
        constraint = is_valid_entry(&entry, &rotator, &snapshot).unwrap() @ ClockworkError::InvalidSnapshotEntry,
    )]
    pub entry: Account<'info, SnapshotEntry>,

    #[account(seeds = [SEED_NODE, node.id.to_be_bytes().as_ref()], bump, constraint = node.id == entry.id)]
    pub node: Account<'info, Node>,

    #[account(address = clockwork_pool_program::ID)]
    pub pool_program: Program<'info, clockwork_pool_program::program::PoolProgram>,

    #[account(seeds = [SEED_CONFIG], bump, seeds::program = clockwork_pool_program::ID)]
    pub pool_program_config: Account<'info, clockwork_pool_program::state::Config>,

    #[account(
        mut, seeds = [SEED_ROTATOR], bump, 
        constraint = Clock::get().unwrap().slot >= rotator.last_rotation_at.checked_add(config.slots_per_rotation).unwrap()
    )]
    pub rotator: Account<'info, Rotator>,

    #[account()]
    pub signer: Signer<'info>,

    #[account(
        seeds = [SEED_SNAPSHOT, snapshot.id.to_be_bytes().as_ref()], bump,
        constraint = snapshot.status == SnapshotStatus::Current @ ClockworkError::SnapshotNotCurrent
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account()]
    pub worker: SystemAccount<'info>,
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, PoolsRotate<'info>>) -> Result<()> {
    // Get accounts
    let node = &ctx.accounts.node;
    let pool_program = &ctx.accounts.pool_program;
    let pool_program_config = &ctx.accounts.pool_program_config;
    let rotator = &mut ctx.accounts.rotator;
    let worker = &ctx.accounts.worker;

    // Require the number of remaining accounts matches the expected number of pools
    require!(rotator.pool_pubkeys.len() == ctx.remaining_accounts.len(), ClockworkError::InvalidPool);

    // Rotate the worker into its supported pools
    let rotator_bump = *ctx.bumps.get("rotator").unwrap();
    for i in 0..ctx.remaining_accounts.len() {
        match ctx.remaining_accounts.get(i) {
            None => return Err(ClockworkError::InvalidPool.into()),
            Some(pool_acc_info) => {

                // Verify the account pubkey is an expected pool
                require!(pool_acc_info.key().eq(rotator.pool_pubkeys.get(i).unwrap()), ClockworkError::InvalidPool);

                // If the node supports this pool, then rotate it in
                if node.supported_pools.contains(&pool_acc_info.key()) {
                    clockwork_pool_program::cpi::pool_rotate(
                        CpiContext::new_with_signer(
                            pool_program.to_account_info(),
                            PoolRotate {
                                config: pool_program_config.to_account_info(),
                                pool: pool_acc_info.clone(),
                                pool_authority: rotator.to_account_info(),
                                worker: worker.to_account_info(),
                            },
                            &[&[SEED_ROTATOR, &[rotator_bump]]],
                        ),
                    )?;
                }
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
