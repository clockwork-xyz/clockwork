use anchor_lang::{prelude::*, solana_program::system_program};
use anchor_spl::{associated_token::get_associated_token_address, token::TokenAccount};
use clockwork_utils::automation::{
    anchor_sighash, AccountBuilder, Ix, AutomationResponse, PAYER_PUBKEY,
};
use std::mem::size_of;

use crate::state::*;

#[derive(Accounts)]
pub struct TakeSnapshotCreateFrame<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

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
            snapshot.id.to_be_bytes().as_ref(),
        ],
        bump,
        constraint = registry.current_epoch.checked_add(1).unwrap().eq(&snapshot.id),
        constraint = snapshot.total_frames < registry.total_workers,
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT_FRAME,
            snapshot.key().as_ref(),
            snapshot.total_frames.to_be_bytes().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<SnapshotFrame>(),
    )]
    pub snapshot_frame: Account<'info, SnapshotFrame>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = config.epoch_automation)]
    pub automation: Signer<'info>,

    #[account(
        address = worker.pubkey(),
        constraint = worker.id.eq(&snapshot.total_frames),
    )]
    pub worker: Account<'info, Worker>,

    #[account(
        associated_token::authority = worker,
        associated_token::mint = config.mint,
    )]
    pub worker_stake: Account<'info, TokenAccount>,
}

pub fn handler(ctx: Context<TakeSnapshotCreateFrame>) -> Result<AutomationResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;
    let snapshot_frame = &mut ctx.accounts.snapshot_frame;
    let system_program = &ctx.accounts.system_program;
    let automation = &ctx.accounts.automation;
    let worker = &ctx.accounts.worker;
    let worker_stake = &ctx.accounts.worker_stake;

    // Initialize snapshot frame account.
    snapshot_frame.init(
        snapshot.total_frames,
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
    snapshot.total_frames = snapshot.total_frames.checked_add(1).unwrap();

    // Build the next instruction for the automation.
    let dynamic_instruction = if worker.total_delegations.gt(&0) {
        // This worker has delegations. Create a snapshot entry for each delegation associated with this worker.
        let zeroth_delegation_pubkey = Delegation::pubkey(worker.pubkey(), 0);
        let zeroth_snapshot_entry_pubkey = SnapshotEntry::pubkey(snapshot_frame.key(), 0);
        Some(Ix {
            program_id: crate::ID,
            accounts: vec![
                AccountBuilder::readonly(config.key(), false),
                AccountBuilder::readonly(zeroth_delegation_pubkey, false),
                AccountBuilder::mutable(PAYER_PUBKEY, true),
                AccountBuilder::readonly(registry.key(), false),
                AccountBuilder::readonly(snapshot.key(), false),
                AccountBuilder::mutable(zeroth_snapshot_entry_pubkey, false),
                AccountBuilder::mutable(snapshot_frame.key(), false),
                AccountBuilder::readonly(system_program.key(), false),
                AccountBuilder::readonly(automation.key(), true),
                AccountBuilder::readonly(worker.key(), false),
            ],
            data: anchor_sighash("take_snapshot_create_entry").to_vec(),
        })
    } else if snapshot.total_frames.lt(&registry.total_workers) {
        // This worker has no delegations. Create a snapshot frame for the next worker.
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
        trigger: None,
    })
}
