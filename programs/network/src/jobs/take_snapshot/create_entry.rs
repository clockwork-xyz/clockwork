use std::mem::size_of;

use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, system_program},
    InstructionData,
};
use anchor_spl::associated_token::get_associated_token_address;
use clockwork_utils::thread::{ThreadResponse, PAYER_PUBKEY};

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

    #[account(address = config.epoch_thread)]
    pub thread: Signer<'info>,

    #[account(
        address = worker.pubkey(),
        constraint = worker.id.eq(&snapshot_frame.id),
    )]
    pub worker: Box<Account<'info, Worker>>,
}

pub fn handler(ctx: Context<TakeSnapshotCreateEntry>) -> Result<ThreadResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let delegation = &ctx.accounts.delegation;
    let registry = &ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;
    let snapshot_entry = &mut ctx.accounts.snapshot_entry;
    let snapshot_frame = &mut ctx.accounts.snapshot_frame;
    let system_program = &ctx.accounts.system_program;
    let thread = &ctx.accounts.thread;
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

    // Build the next instruction for the thread.
    let dynamic_instruction = if snapshot_frame.total_entries.lt(&worker.total_delegations) {
        // Create a snapshot entry for the next delegation.
        let next_delegation_pubkey =
            Delegation::pubkey(worker.pubkey(), delegation.id.checked_add(1).unwrap());
        let next_snapshot_entry_pubkey = SnapshotEntry::pubkey(
            snapshot_frame.key(),
            snapshot_entry.id.checked_add(1).unwrap(),
        );
        Some(
            Instruction {
                program_id: crate::ID,
                accounts: crate::accounts::TakeSnapshotCreateEntry {
                    config: config.key(),
                    delegation: next_delegation_pubkey,
                    payer: PAYER_PUBKEY,
                    registry: registry.key(),
                    snapshot: snapshot.key(),
                    snapshot_entry: next_snapshot_entry_pubkey,
                    snapshot_frame: snapshot_frame.key(),
                    system_program: system_program.key(),
                    thread: thread.key(),
                    worker: worker.key(),
                }
                .to_account_metas(Some(true)),
                data: crate::instruction::TakeSnapshotCreateEntry {}.data(),
            }
            .into(),
        )
    } else if snapshot.total_frames.lt(&registry.total_workers) {
        // This frame has captured all its entries. Create a frame for the next worker.
        let next_snapshot_frame_pubkey =
            SnapshotFrame::pubkey(snapshot.key(), snapshot_frame.id.checked_add(1).unwrap());
        let next_worker_pubkey = Worker::pubkey(worker.id.checked_add(1).unwrap());
        Some(
            Instruction {
                program_id: crate::ID,
                accounts: crate::accounts::TakeSnapshotCreateFrame {
                    config: config.key(),
                    payer: PAYER_PUBKEY,
                    registry: registry.key(),
                    snapshot: snapshot.key(),
                    snapshot_frame: next_snapshot_frame_pubkey,
                    system_program: system_program.key(),
                    thread: thread.key(),
                    worker: next_worker_pubkey,
                    worker_stake: get_associated_token_address(&next_worker_pubkey, &config.mint),
                }
                .to_account_metas(Some(true)),
                data: crate::instruction::TakeSnapshotCreateFrame {}.data(),
            }
            .into(),
        )
    } else {
        None
    };

    Ok(ThreadResponse {
        dynamic_instruction,
        ..ThreadResponse::default()
    })
}
