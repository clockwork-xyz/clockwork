use {
    crate::{
        errors::ClockworkError,
        state::*
    },
    anchor_lang::prelude::*,
    clockwork_pool::cpi::accounts::Rotate
};

#[derive(Accounts)]
pub struct RotatorTurn<'info> {
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

    #[account(mut, address = clockwork_pool::state::Pool::pubkey())]
    pub pool: Account<'info, clockwork_pool::state::Pool>,

    #[account(address = clockwork_pool::state::Config::pubkey())]
    pub pool_config: Account<'info, clockwork_pool::state::Config>,

    #[account(address = clockwork_pool::ID)]
    pub pool_program: Program<'info, clockwork_pool::program::ClockworkPool>,

    #[account(
        mut, 
        seeds = [SEED_ROTATOR], 
        bump, 
        constraint = Clock::get().unwrap().slot >= rotator.last_slot.checked_add(config.slots_per_rotation).unwrap()
    )]
    pub rotator: Account<'info, Rotator>,

    #[account()]
    pub signer: Signer<'info>,

    #[account(
        seeds = [
            SEED_SNAPSHOT, 
            snapshot.id.to_be_bytes().as_ref()
        ], 
        bump,
        constraint = snapshot.status == SnapshotStatus::Current @ ClockworkError::SnapshotNotCurrent
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account()]
    pub worker: SystemAccount<'info>,
}

pub fn handler(ctx: Context<RotatorTurn>) -> Result<()> {
    // Get accounts
    let pool = &mut ctx.accounts.pool;
    let pool_config = &ctx.accounts.pool_config;
    let pool_program = &ctx.accounts.pool_program;
    let rotator = &mut ctx.accounts.rotator;
    let worker = &ctx.accounts.worker;

    // TODO Slash stakes of current workers if rotator is too many slots behind

    // Rotate the pool and hash the nonce
    let rotator_bump = *ctx.bumps.get("rotator").unwrap();
    clockwork_pool::cpi::rotate(
        CpiContext::new_with_signer(
            pool_program.to_account_info(),
            Rotate {
                config: pool_config.to_account_info(),
                rotator: rotator.to_account_info(),
                pool: pool.to_account_info(),
                worker: worker.to_account_info(),
            },
            &[&[SEED_ROTATOR, &[rotator_bump]]],
        ),
    )?;

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
