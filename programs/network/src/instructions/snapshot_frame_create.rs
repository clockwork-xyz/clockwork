use {
    crate::{errors::ClockworkError, objects::*},
    anchor_lang::{prelude::*, solana_program::system_program},
    anchor_spl::{associated_token::get_associated_token_address, token::TokenAccount},
    clockwork_utils::{anchor_sighash, AccountMetaData, CrankResponse, InstructionData},
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct SnapshotFrameCreate<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = current_epoch.pubkey(),
        constraint = current_epoch.current
    )]
    pub current_epoch: Account<'info, Epoch>,

    #[account(
        address = epoch.pubkey(),
        constraint = current_epoch.id.checked_add(1).unwrap().eq(&epoch.id),
    )]
    pub epoch: Account<'info, Epoch>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        address = Registry::pubkey(),
        constraint = registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            snapshot.epoch.as_ref(),
        ],
        bump,
        has_one = epoch,
        constraint = snapshot.total_workers < registry.total_workers,
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT_FRAME,
            snapshot.key().as_ref(),
            snapshot.total_workers.to_be_bytes().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<SnapshotFrame>(),
    )]
    pub snapshot_frame: Account<'info, SnapshotFrame>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(
        address = worker.pubkey(),
        constraint = worker.id.eq(&snapshot.total_workers),
    )]
    pub worker: Account<'info, Worker>,

    #[account(
        associated_token::authority = worker,
        associated_token::mint = config.mint,
    )]
    pub worker_stake: Account<'info, TokenAccount>,
}

pub fn handler(ctx: Context<SnapshotFrameCreate>) -> Result<CrankResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let current_epoch = &ctx.accounts.current_epoch;
    let epoch = &ctx.accounts.epoch;
    let payer = &ctx.accounts.payer;
    let registry = &ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;
    let snapshot_frame = &mut ctx.accounts.snapshot_frame;
    let system_program = &ctx.accounts.system_program;
    let worker = &ctx.accounts.worker;
    let worker_stake = &ctx.accounts.worker_stake;

    // Initialize snapshot frame account.
    snapshot_frame.init(
        snapshot.total_workers,
        snapshot.key(),
        worker_stake.amount,
        snapshot.total_stake,
        worker.key(),
    )?;

    // Update snapshot total workers.
    snapshot.total_stake = snapshot
        .total_stake
        .checked_add(worker_stake.amount)
        .unwrap();
    snapshot.total_workers = snapshot.total_workers.checked_add(1).unwrap();

    // Build the next instruction for the queue.
    let next_instruction = if worker.total_delegations > 0 {
        // This worker has delegations. Create snapshot entries for each delegation associated with this worker.
        let zeroth_delegation_pubkey = Delegation::pubkey(worker.pubkey(), 0);
        let zeroth_snapshot_entry_pubkey = SnapshotEntry::pubkey(snapshot_frame.key(), 0);
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(current_epoch.key(), false),
                AccountMetaData::new_readonly(zeroth_delegation_pubkey, false),
                AccountMetaData::new_readonly(
                    get_associated_token_address(&zeroth_delegation_pubkey, &config.mint),
                    false,
                ),
                AccountMetaData::new_readonly(epoch.key(), false),
                AccountMetaData::new(payer.key(), true),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(snapshot.key(), false),
                AccountMetaData::new(zeroth_snapshot_entry_pubkey, false),
                AccountMetaData::new(snapshot_frame.key(), false),
                AccountMetaData::new_readonly(system_program.key(), false),
                AccountMetaData::new_readonly(worker.key(), false),
            ],
            data: anchor_sighash("snapshot_entry_create").to_vec(),
        })
    } else if snapshot.total_workers.lt(&registry.total_workers) {
        // This worker has no delegations. Create a snapshot frame for the next worker.
        let next_snapshot_frame_pubkey =
            SnapshotFrame::pubkey(snapshot_frame.id.checked_add(1).unwrap(), snapshot.key());
        let next_worker_pubkey = Worker::pubkey(worker.id.checked_add(1).unwrap());
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(current_epoch.key(), false),
                AccountMetaData::new_readonly(epoch.key(), false),
                AccountMetaData::new(payer.key(), true),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new(snapshot.key(), false),
                AccountMetaData::new(next_snapshot_frame_pubkey, false),
                AccountMetaData::new_readonly(system_program.key(), false),
                AccountMetaData::new_readonly(next_worker_pubkey, false),
                AccountMetaData::new_readonly(
                    get_associated_token_address(&next_worker_pubkey, &config.mint),
                    false,
                ),
            ],
            data: anchor_sighash("snapshot_frame_create").to_vec(),
        })
    } else if snapshot.total_workers.eq(&registry.total_workers) {
        // TODO The snapshot is done!
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                // TODO AccountMetaData::new(pubkey, is_signer)
            ],
            data: anchor_sighash("epoch_start").to_vec(),
        })
    } else {
        // Something is wrong...
        return Err(ClockworkError::InvalidSnapshot.into());
    };

    Ok(CrankResponse { next_instruction })
}
