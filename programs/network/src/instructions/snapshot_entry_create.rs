use {
    crate::{errors::ClockworkError, objects::*},
    anchor_lang::{prelude::*, solana_program::system_program},
    anchor_spl::{associated_token::get_associated_token_address, token::TokenAccount},
    clockwork_utils::{anchor_sighash, AccountMetaData, CrankResponse, InstructionData},
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct SnapshotEntryCreate<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = current_epoch.pubkey(),
        constraint = current_epoch.id.eq(&registry.current_epoch_id)
    )]
    pub current_epoch: Account<'info, Epoch>,

    #[account(
        address = delegation.pubkey(),
        constraint = delegation.id.eq(&snapshot_frame.total_entries),
        has_one = worker,
    )]
    pub delegation: Account<'info, Delegation>,

    #[account(
        associated_token::authority = delegation,
        associated_token::mint = config.mint,
    )]
    pub delegation_stake: Account<'info, TokenAccount>,

    #[account(
        address = epoch.pubkey(),
        constraint = current_epoch.id.checked_add(1).unwrap().eq(&epoch.id),
    )]
    pub epoch: Account<'info, Epoch>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = config.authorized_queue)]
    pub queue: Signer<'info>,

    #[account(
        address = Registry::pubkey(),
        constraint = registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(
        address = snapshot.pubkey(),
        has_one = epoch,
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT_ENTRY,
            snapshot_frame.key().as_ref(),
            snapshot_frame.total_entries.to_be_bytes().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<SnapshotEntry>(),
    )]
    pub snapshot_entry: Account<'info, SnapshotEntry>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT_FRAME,
            snapshot_frame.snapshot.as_ref(),
            snapshot_frame.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = snapshot,
        constraint = snapshot_frame.id.eq(&snapshot.total_frames),
    )]
    pub snapshot_frame: Account<'info, SnapshotFrame>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(
        address = worker.pubkey(),
        constraint = worker.id.eq(&snapshot_frame.id),
    )]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<SnapshotEntryCreate>) -> Result<CrankResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let current_epoch = &ctx.accounts.current_epoch;
    let delegation = &ctx.accounts.delegation;
    let delegation_stake = &ctx.accounts.delegation_stake;
    let epoch = &ctx.accounts.epoch;
    let payer = &ctx.accounts.payer;
    let queue = &ctx.accounts.queue;
    let registry = &ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;
    let snapshot_entry = &mut ctx.accounts.snapshot_entry;
    let snapshot_frame = &mut ctx.accounts.snapshot_frame;
    let system_program = &ctx.accounts.system_program;
    let worker = &ctx.accounts.worker;

    // Initialize snapshot entry account.
    snapshot_entry.init(
        delegation.key(),
        snapshot_frame.total_entries,
        snapshot_frame.key(),
        delegation_stake.amount,
    )?;

    // Update the snapshot frame.
    snapshot_frame.total_entries = snapshot_frame.total_entries.checked_add(1).unwrap();

    // Build the next instruction for the queue.
    let next_instruction = if snapshot_frame.total_entries.lt(&worker.total_delegations) {
        // Create a snapshot entry for the next delegation.
        let next_delegation_pubkey =
            Delegation::pubkey(worker.pubkey(), delegation.id.checked_add(1).unwrap());
        let next_snapshot_entry_pubkey = SnapshotEntry::pubkey(
            snapshot_frame.key(),
            snapshot_entry.id.checked_add(1).unwrap(),
        );
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(current_epoch.key(), false),
                AccountMetaData::new_readonly(next_delegation_pubkey, false),
                AccountMetaData::new_readonly(
                    get_associated_token_address(&next_delegation_pubkey, &config.mint),
                    false,
                ),
                AccountMetaData::new_readonly(epoch.key(), false),
                AccountMetaData::new(payer.key(), true),
                AccountMetaData::new_readonly(queue.key(), true),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(snapshot.key(), false),
                AccountMetaData::new(next_snapshot_entry_pubkey, false),
                AccountMetaData::new(snapshot_frame.key(), false),
                AccountMetaData::new_readonly(system_program.key(), false),
                AccountMetaData::new_readonly(worker.key(), false),
            ],
            data: anchor_sighash("snapshot_entry_create").to_vec(),
        })
    } else if snapshot_frame.total_entries.eq(&worker.total_delegations)
        && snapshot.total_frames.lt(&registry.total_workers)
    {
        // This frame has captured all its entries. Create a frame for the next worker.
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
                AccountMetaData::new_readonly(queue.key(), true),
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
    } else if snapshot_frame.total_entries.eq(&worker.total_delegations)
        && snapshot.total_frames.eq(&registry.total_workers)
    {
        // All entries in this frame have been captured, and it is the last frame. The snapshot is done!
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new(current_epoch.key(), false),
                AccountMetaData::new(epoch.key(), false),
                AccountMetaData::new_readonly(queue.key(), true),
            ],
            data: anchor_sighash("epoch_start").to_vec(),
        })
    } else {
        // Something is wrong...
        return Err(ClockworkError::InvalidSnapshot.into());
    };

    Ok(CrankResponse { next_instruction })
}
