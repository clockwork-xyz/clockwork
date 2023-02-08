use anchor_lang::{prelude::*, solana_program::system_program};
use anchor_spl::associated_token::get_associated_token_address;
use clockwork_utils::automation::{
    anchor_sighash, AccountBuilder, Ix, AutomationResponse, PAYER_PUBKEY,
};
use std::mem::size_of;

use crate::state::*;

#[derive(Accounts)]
pub struct TakeSnapshotCreateEntry<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = delegation.pubkey(),
        constraint = delegation.id.eq(&snapshot_frame.total_entries),
        has_one = worker,
    )]
    pub delegation: Box<Account<'info, Delegation>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        address = Registry::pubkey(),
        constraint = registry.locked
    )]
    pub registry: Box<Account<'info, Registry>>,

    #[account(
        address = snapshot.pubkey(),
        constraint = registry.current_epoch.checked_add(1).unwrap().eq(&snapshot.id)
    )]
    pub snapshot: Box<Account<'info, Snapshot>>,

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
        constraint = snapshot_frame.id.checked_add(1).unwrap().eq(&snapshot.total_frames),
    )]
    pub snapshot_frame: Box<Account<'info, SnapshotFrame>>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = config.epoch_automation)]
    pub automation: Signer<'info>,

    #[account(
        address = worker.pubkey(),
        constraint = worker.id.eq(&snapshot_frame.id),
    )]
    pub worker: Box<Account<'info, Worker>>,
}

pub fn handler(ctx: Context<TakeSnapshotCreateEntry>) -> Result<AutomationResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let delegation = &ctx.accounts.delegation;
    let registry = &ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;
    let snapshot_entry = &mut ctx.accounts.snapshot_entry;
    let snapshot_frame = &mut ctx.accounts.snapshot_frame;
    let system_program = &ctx.accounts.system_program;
    let automation = &ctx.accounts.automation;
    let worker = &ctx.accounts.worker;

    // Initialize snapshot entry account.
    snapshot_entry.init(
        delegation.key(),
        snapshot_frame.total_entries,
        snapshot_frame.key(),
        delegation.stake_amount,
    )?;

    // Update the snapshot frame.
    snapshot_frame.total_entries = snapshot_frame.total_entries.checked_add(1).unwrap();

    // Build the next instruction for the automation.
    let dynamic_instruction = if snapshot_frame.total_entries.lt(&worker.total_delegations) {
        // Create a snapshot entry for the next delegation.
        let next_delegation_pubkey =
            Delegation::pubkey(worker.pubkey(), delegation.id.checked_add(1).unwrap());
        let next_snapshot_entry_pubkey = SnapshotEntry::pubkey(
            snapshot_frame.key(),
            snapshot_entry.id.checked_add(1).unwrap(),
        );
        Some(Ix {
            program_id: crate::ID,
            accounts: vec![
                AccountBuilder::readonly(config.key(), false),
                AccountBuilder::readonly(next_delegation_pubkey, false),
                AccountBuilder::mutable(PAYER_PUBKEY, true),
                AccountBuilder::readonly(automation.key(), true),
                AccountBuilder::readonly(registry.key(), false),
                AccountBuilder::readonly(snapshot.key(), false),
                AccountBuilder::mutable(next_snapshot_entry_pubkey, false),
                AccountBuilder::mutable(snapshot_frame.key(), false),
                AccountBuilder::readonly(system_program.key(), false),
                AccountBuilder::readonly(worker.key(), false),
            ],
            data: anchor_sighash("take_snapshot_create_entry").to_vec(),
        })
    } else if snapshot.total_frames.lt(&registry.total_workers) {
        // This frame has captured all its entries. Create a frame for the next worker.
        let next_snapshot_frame_pubkey =
            SnapshotFrame::pubkey(snapshot.key(), snapshot_frame.id.checked_add(1).unwrap());
        let next_worker_pubkey = Worker::pubkey(worker.id.checked_add(1).unwrap());
        Some(Ix {
            program_id: crate::ID,
            accounts: vec![
                AccountBuilder::readonly(config.key(), false),
                AccountBuilder::mutable(PAYER_PUBKEY, true),
                AccountBuilder::readonly(registry.key(), false),
                AccountBuilder::mutable(snapshot.key(), false),
                AccountBuilder::mutable(next_snapshot_frame_pubkey, false),
                AccountBuilder::readonly(system_program.key(), false),
                AccountBuilder::readonly(automation.key(), true),
                AccountBuilder::readonly(next_worker_pubkey, false),
                AccountBuilder::readonly(
                    get_associated_token_address(&next_worker_pubkey, &config.mint),
                    false,
                ),
            ],
            data: anchor_sighash("take_snapshot_create_frame").to_vec(),
        })
    } else {
        None
    };

    Ok(AutomationResponse {
        dynamic_instruction,
        ..AutomationResponse::default()
    })
}
