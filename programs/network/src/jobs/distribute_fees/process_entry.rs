use anchor_lang::{prelude::*, InstructionData};
use clockwork_utils::automation::{AutomationResponse, InstructionBuilder};

use crate::{instruction, state::*};

#[derive(Accounts)]
pub struct DistributeFeesProcessEntry<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_DELEGATION,
            delegation.worker.as_ref(),
            delegation.id.to_be_bytes().as_ref(),
        ],
        bump,
        constraint = delegation.id.eq(&snapshot_entry.id),
        has_one = worker,
    )]
    pub delegation: Account<'info, Delegation>,

    #[account(
        mut,
        seeds = [
            SEED_FEE,
            fee.worker.as_ref(),
        ],
        bump,
        has_one = worker,
    )]
    pub fee: Account<'info, Fee>,

    #[account(address = Registry::pubkey())]
    pub registry: Account<'info, Registry>,

    #[account(
        address = snapshot.pubkey(),
        constraint = registry.current_epoch.eq(&registry.current_epoch)
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(
        address = snapshot_entry.pubkey(),
        has_one = snapshot_frame,
    )]
    pub snapshot_entry: Account<'info, SnapshotEntry>,

    #[account(
        address = snapshot_frame.pubkey(),
        has_one = snapshot,
        has_one = worker,
    )]
    pub snapshot_frame: Account<'info, SnapshotFrame>,

    #[account(address = config.epoch_automation)]
    pub automation: Signer<'info>,

    #[account(address = worker.pubkey())]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<DistributeFeesProcessEntry>) -> Result<AutomationResponse> {
    // Get accounts
    let config = &ctx.accounts.config;
    let delegation = &mut ctx.accounts.delegation;
    let fee = &mut ctx.accounts.fee;
    let registry = &ctx.accounts.registry;
    let snapshot = &ctx.accounts.snapshot;
    let snapshot_entry = &ctx.accounts.snapshot_entry;
    let snapshot_frame = &ctx.accounts.snapshot_frame;
    let automation = &ctx.accounts.automation;
    let worker = &ctx.accounts.worker;

    // Calculate the balance of this particular delegation, based on the weight of its stake with this worker.
    let distribution_balance = if snapshot_frame.stake_amount.gt(&0) {
        fee.distributable_balance
            .checked_mul(snapshot_entry.stake_amount)
            .unwrap()
            .checked_div(snapshot_frame.stake_amount)
            .unwrap()
    } else {
        0
    };

    // Transfer yield to the worker.
    **fee.to_account_info().try_borrow_mut_lamports()? = fee
        .to_account_info()
        .lamports()
        .checked_sub(distribution_balance)
        .unwrap();
    **delegation.to_account_info().try_borrow_mut_lamports()? = delegation
        .to_account_info()
        .lamports()
        .checked_add(distribution_balance)
        .unwrap();

    // Increment the delegation's yield balance.
    delegation.yield_balance = delegation
        .yield_balance
        .checked_add(distribution_balance)
        .unwrap();

    // Build the next instruction for the automation.
    let dynamic_instruction = if snapshot_entry
        .id
        .checked_add(1)
        .unwrap()
        .lt(&snapshot_frame.total_entries)
    {
        // This frame has more entries. Move on to the next one.
        let next_delegation_pubkey =
            Delegation::pubkey(worker.key(), delegation.id.checked_add(1).unwrap());
        let next_snapshot_entry_pubkey = SnapshotEntry::pubkey(
            snapshot_frame.key(),
            snapshot_entry.id.checked_add(1).unwrap(),
        );
        Some(
            InstructionBuilder::new(crate::ID)
                .readonly_account(config.key())
                .mutable_account(next_delegation_pubkey)
                .mutable_account(fee.key())
                .readonly_account(registry.key())
                .readonly_account(snapshot.key())
                .readonly_account(next_snapshot_entry_pubkey)
                .readonly_account(snapshot_frame.key())
                .signer(automation.key())
                .readonly_account(worker.key())
                .data(instruction::DistributeFeesProcessEntry {}.data())
                .build(),
        )
    } else if snapshot_frame
        .id
        .checked_add(1)
        .unwrap()
        .lt(&snapshot.total_frames)
    {
        // This frame has no more entries. Move on to the next worker.
        let next_worker_pubkey = Worker::pubkey(worker.id.checked_add(1).unwrap());
        let next_snapshot_frame_pubkey =
            SnapshotFrame::pubkey(snapshot.key(), snapshot_frame.id.checked_add(1).unwrap());
        Some(
            InstructionBuilder::new(crate::ID)
                .readonly_account(config.key())
                .mutable_account(Fee::pubkey(next_worker_pubkey))
                .readonly_account(registry.key())
                .readonly_account(snapshot.key())
                .readonly_account(next_snapshot_frame_pubkey)
                .signer(automation.key())
                .mutable_account(next_worker_pubkey)
                .data(instruction::DistributeFeesProcessFrame {}.data())
                .build(),
        )
    } else {
        None
    };

    Ok(AutomationResponse {
        dynamic_instruction,
        trigger: None,
    })
}
