use cronos_pool::cpi::accounts::Cycle;

use crate::errors::CronosError;

use {crate::state::*, anchor_lang::{prelude::*, solana_program::sysvar}};

#[derive(Accounts)]
pub struct CyclerRun<'info> {

    #[account(
        address = sysvar::clock::ID, 
        constraint = clock.slot >= cycler.last_cycle_at + config.slots_per_cycle
    )]
    pub clock: Sysvar<'info, Clock>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Account<'info, Config>,

    #[account(mut, seeds = [SEED_CYCLER], bump)]
    pub cycler: Account<'info, Cycler>,

    #[account(
        seeds = [
            SEED_SNAPSHOT_ENTRY,
            entry.snapshot.as_ref(),
            entry.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = snapshot,
    )]
    pub entry: Account<'info, SnapshotEntry>,

    #[account(mut, address = cronos_pool::state::Pool::pda().0)]
    pub pool: Account<'info, cronos_pool::state::Pool>,

    #[account(address = cronos_pool::state::Config::pda().0)]
    pub pool_config: Account<'info, cronos_pool::state::Config>,

    #[account(address = cronos_pool::ID)]
    pub pool_program: Program<'info, cronos_pool::program::CronosPool>,

    #[account(seeds = [SEED_REGISTRY], bump)]
    pub registry: Account<'info, Registry>,

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

pub fn handler(ctx: Context<CyclerRun>) -> Result<()> {
    // Get accounts
    let clock = &ctx.accounts.clock;
    let cycler = &mut ctx.accounts.cycler;
    let entry = &ctx.accounts.entry;
    let pool = &mut ctx.accounts.pool;
    let pool_config = &ctx.accounts.pool_config;
    let pool_program = &ctx.accounts.pool_program;
    // let registry = &ctx.accounts.registry;
    let snapshot = &ctx.accounts.snapshot;

    

    // msg!("Cycler: {}", cycler.entry_id);
    // msg!(
    //     "Snapshot: {} {:?} {} {}",
    //     snapshot.id,
    //     snapshot.status,
    //     snapshot.entry_count,
    //     snapshot.stake_total
    // );
    // msg!(
    //     "Entry: {} {} {} {}",
    //     entry.id,
    //     entry.delegate,
    //     entry.stake_offset,
    //     entry.stake_amount
    // );

    // Quietly noop if the entry ids aren't aligned
    // if cycler.entry_id != entry.id {
    //     return Ok(ExecResponse::default());
    // }

    // Rotate the cycler's entry id counter
    // cycler.entry_id = cycler
    //     .entry_id
    //     .checked_add(1)
    //     .unwrap()
    //     .checked_rem(snapshot.entry_count)
    //     .unwrap();

    // If the snapshot is not current, then return early and invoke next time
    //  with the current snapshot and entry accounts
    // if snapshot.status != SnapshotStatus::Current {
    //     let entry_pubkey = entry.key();
    //     let snapshot_pubkey = snapshot.key();
    //     let current_snapshot_pubkey =
    //         Snapshot::pda(registry.snapshot_count.checked_sub(1).unwrap()).0;
    //     let current_entry_pubky = SnapshotEntry::pda(current_snapshot_pubkey, entry.id).0;
    //     return Ok(ExecResponse {
    //         dynamic_accounts: Some(
    //             ctx.accounts
    //                 .to_account_metas(None)
    //                 .iter()
    //                 .map(|acc| match acc.pubkey {
    //                     _ if acc.pubkey == entry_pubkey => current_entry_pubky,
    //                     _ if acc.pubkey == snapshot_pubkey => current_snapshot_pubkey,
    //                     _ => acc.pubkey,
    //                 })
    //                 .collect(),
    //         ),
    //     });
    // }


    // TODO Slash stakes of current delegates if cycler is late.

    // Return early if the wrong entry was provided.
    if !cycler.is_valid_entry(entry, snapshot)? {
        return Err(CronosError::InvalidSnapshotEntry.into());
    }

    // If the delegate sample is valid, then cycle the pool and hash the nonce
    let cycler_bump = *ctx.bumps.get("cycler").unwrap();
    cronos_pool::cpi::cycle(
        CpiContext::new_with_signer(
            pool_program.to_account_info(),
            Cycle {
                config: pool_config.to_account_info(),
                cycler: cycler.to_account_info(),
                pool: pool.to_account_info(),
            },
            &[&[SEED_CYCLER, &[cycler_bump]]],
        ),
        entry.delegate,
    )?;

    // Hash the cycler nonce
    cycler.hash_nonce(clock.slot)?;

    Ok(())
}
