use cronos_pool::cpi::accounts::Rotate;

use crate::errors::CronosError;

use {crate::state::*, anchor_lang::{prelude::*, solana_program::sysvar}};

#[derive(Accounts)]
pub struct RotatorTurn<'info> {
    #[account(
        address = sysvar::clock::ID, 
        constraint = clock.slot >= rotator.last_slot.checked_add(config.slots_per_rotation).unwrap()
    )]
    pub clock: Sysvar<'info, Clock>,

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
        constraint = is_valid_entry(&entry, &rotator, &snapshot).unwrap() @ CronosError::InvalidSnapshotEntry,
    )]
    pub entry: Account<'info, SnapshotEntry>,

    #[account(mut, address = cronos_pool::state::Pool::pda().0)]
    pub pool: Account<'info, cronos_pool::state::Pool>,

    #[account(address = cronos_pool::state::Config::pda().0)]
    pub pool_config: Account<'info, cronos_pool::state::Config>,

    #[account(address = cronos_pool::ID)]
    pub pool_program: Program<'info, cronos_pool::program::CronosPool>,

    #[account(mut, seeds = [SEED_ROTATOR], bump)]
    pub rotator: Account<'info, Rotator>,

    #[account()]
    pub signer: Signer<'info>,

    #[account(
        seeds = [
            SEED_SNAPSHOT, 
            snapshot.id.to_be_bytes().as_ref()
        ], 
        bump,
        constraint = snapshot.status == SnapshotStatus::Current @ CronosError::SnapshotNotCurrent
    )]
    pub snapshot: Account<'info, Snapshot>,
}

pub fn handler(ctx: Context<RotatorTurn>) -> Result<()> {
    // Get accounts
    let clock = &ctx.accounts.clock;
    let entry = &ctx.accounts.entry;
    let pool = &mut ctx.accounts.pool;
    let pool_config = &ctx.accounts.pool_config;
    let pool_program = &ctx.accounts.pool_program;
    let rotator = &mut ctx.accounts.rotator;

    // TODO Slash stakes of current delegates if rotator is too many slots behind

    // Rotate the pool and hash the nonce
    let rotator_bump = *ctx.bumps.get("rotator").unwrap();
    cronos_pool::cpi::rotate(
        CpiContext::new_with_signer(
            pool_program.to_account_info(),
            Rotate {
                config: pool_config.to_account_info(),
                rotator: rotator.to_account_info(),
                pool: pool.to_account_info(),
            },
            &[&[SEED_ROTATOR, &[rotator_bump]]],
        ),
        entry.delegate,
    )?;

    // Hash the rotator's nonce value
    rotator.hash_nonce(clock.slot)?;

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