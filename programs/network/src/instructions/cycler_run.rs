use cronos_pool::cpi::accounts::Cycle;

use {
    crate::state::*,
    anchor_lang::prelude::*,
    cronos_scheduler::{responses::ExecResponse, state::Yogi},
};

#[derive(Accounts)]
pub struct CyclerRun<'info> {
    #[account(seeds = [SEED_AUTHORITY], bump, has_one = yogi)]
    pub authority: Account<'info, Authority>,

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

    #[account(signer, constraint = yogi.owner == authority.key())]
    pub yogi: Account<'info, Yogi>,

    #[account(seeds = [SEED_REGISTRY], bump)]
    pub registry: Account<'info, Registry>,

    #[account(seeds = [SEED_SNAPSHOT, snapshot.id.to_be_bytes().as_ref()], bump)]
    pub snapshot: Account<'info, Snapshot>,
}

pub fn handler(ctx: Context<CyclerRun>) -> Result<ExecResponse> {
    // Get accounts
    let cycler = &mut ctx.accounts.cycler;
    let entry = &ctx.accounts.entry;
    let pool = &mut ctx.accounts.pool;
    let pool_config = &ctx.accounts.pool_config;
    let pool_program = &ctx.accounts.pool_program;
    let registry = &ctx.accounts.registry;
    let snapshot = &ctx.accounts.snapshot;

    msg!("Cycler: {}", cycler.entry_id);
    msg!(
        "Snapshot: {} {:?} {} {}",
        snapshot.id,
        snapshot.status,
        snapshot.entry_count,
        snapshot.stake_total
    );
    msg!(
        "Entry: {} {} {} {}",
        entry.id,
        entry.delegate,
        entry.stake_offset,
        entry.stake_amount
    );

    // Quietly noop if the entry ids aren't aligned
    if cycler.entry_id != entry.id {
        return Ok(ExecResponse::default());
    }

    // Rotate the cycler's entry id counter
    cycler.entry_id = cycler
        .entry_id
        .checked_add(1)
        .unwrap()
        .checked_rem(snapshot.entry_count)
        .unwrap();

    // If the snapshot is not current, then return early and invoke next time
    //  with the current snapshot and entry accounts
    if snapshot.status != SnapshotStatus::Current {
        let entry_pubkey = entry.key();
        let snapshot_pubkey = snapshot.key();
        let current_snapshot_pubkey =
            Snapshot::pda(registry.snapshot_count.checked_sub(1).unwrap()).0;
        let current_entry_pubky = SnapshotEntry::pda(current_snapshot_pubkey, entry.id).0;
        return Ok(ExecResponse {
            dynamic_accounts: Some(
                ctx.accounts
                    .to_account_metas(None)
                    .iter()
                    .map(|acc| match acc.pubkey {
                        _ if acc.pubkey == entry_pubkey => current_entry_pubky,
                        _ if acc.pubkey == snapshot_pubkey => current_snapshot_pubkey,
                        _ => acc.pubkey,
                    })
                    .collect(),
            ),
        });
    }

    // If the delegate sample is valid, then cycle the pool and hash the nonce
    let cycler_bump = *ctx.bumps.get("cycler").unwrap();
    if cycler.is_valid_delegate(entry, snapshot)? {
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

        cycler.hash_nonce()?;
    }

    Ok(ExecResponse::default())
}
