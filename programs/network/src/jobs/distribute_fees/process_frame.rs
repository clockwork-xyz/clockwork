use anchor_lang::{prelude::*, InstructionData};
use clockwork_utils::automation::{
    AutomationResponse, SerializableAccount, SerializableInstruction,
};

use crate::{instruction, state::*};

#[derive(Accounts)]
pub struct DistributeFeesProcessFrame<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

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
        constraint = snapshot.id.eq(&registry.current_epoch)
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(
        address = snapshot_frame.pubkey(),
        has_one = snapshot,
        has_one = worker,
    )]
    pub snapshot_frame: Account<'info, SnapshotFrame>,

    #[account(address = config.epoch_automation)]
    pub automation: Signer<'info>,

    #[account(mut)]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<DistributeFeesProcessFrame>) -> Result<AutomationResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let fee = &mut ctx.accounts.fee;
    let registry = &ctx.accounts.registry;
    let snapshot = &ctx.accounts.snapshot;
    let snapshot_frame = &ctx.accounts.snapshot_frame;
    let automation = &ctx.accounts.automation;
    let worker = &mut ctx.accounts.worker;

    // Calculate the fee account's usuable balance.
    let fee_lamport_balance = fee.to_account_info().lamports();
    let fee_data_len = 8 + fee.try_to_vec()?.len();
    let fee_rent_balance = Rent::get().unwrap().minimum_balance(fee_data_len);
    let fee_usable_balance = fee_lamport_balance.checked_sub(fee_rent_balance).unwrap();

    // Calculate the commission to be retained by the worker.
    let commission_balance = fee_usable_balance
        .checked_mul(worker.commission_rate)
        .unwrap()
        .checked_div(100)
        .unwrap();

    // Transfer commission to the worker.
    **fee.to_account_info().try_borrow_mut_lamports()? = fee
        .to_account_info()
        .lamports()
        .checked_sub(commission_balance)
        .unwrap();
    **worker.to_account_info().try_borrow_mut_lamports()? = worker
        .to_account_info()
        .lamports()
        .checked_add(commission_balance)
        .unwrap();

    // Increment the worker's commission balance.
    worker.commission_balance = worker
        .commission_balance
        .checked_add(commission_balance)
        .unwrap();

    // Record the balance that is distributable to delegations.
    fee.distributable_balance = fee_usable_balance.checked_sub(commission_balance).unwrap();

    // Build next instruction for the automation.
    let dynamic_instruction = if snapshot_frame.total_entries.gt(&0) {
        // This snapshot frame has entries. Distribute fees to the delegations associated with the entries.
        let delegation_pubkey = Delegation::pubkey(worker.key(), 0);
        let snapshot_entry_pubkey = SnapshotEntry::pubkey(snapshot_frame.key(), 0);
        Some(SerializableInstruction {
            program_id: crate::ID,
            accounts: vec![
                SerializableAccount::readonly(config.key(), false),
                SerializableAccount::mutable(delegation_pubkey, false),
                SerializableAccount::mutable(fee.key(), false),
                SerializableAccount::readonly(registry.key(), false),
                SerializableAccount::readonly(snapshot.key(), false),
                SerializableAccount::readonly(snapshot_entry_pubkey.key(), false),
                SerializableAccount::readonly(snapshot_frame.key(), false),
                SerializableAccount::readonly(automation.key(), true),
                SerializableAccount::readonly(worker.key(), false),
            ],
            data: instruction::DistributeFeesProcessEntry {}.data(),
        })
    } else if snapshot_frame
        .id
        .checked_add(1)
        .unwrap()
        .lt(&snapshot.total_frames)
    {
        // This frame has no entries. Move on to the next frame.
        let next_worker_pubkey = Worker::pubkey(worker.id.checked_add(1).unwrap());
        let next_snapshot_frame_pubkey =
            SnapshotFrame::pubkey(snapshot.key(), snapshot_frame.id.checked_add(1).unwrap());
        Some(SerializableInstruction {
            program_id: crate::ID,
            accounts: vec![
                SerializableAccount::readonly(config.key(), false),
                SerializableAccount::mutable(Fee::pubkey(next_worker_pubkey), false),
                SerializableAccount::readonly(registry.key(), false),
                SerializableAccount::readonly(snapshot.key(), false),
                SerializableAccount::readonly(next_snapshot_frame_pubkey, false),
                SerializableAccount::readonly(automation.key(), true),
                SerializableAccount::mutable(next_worker_pubkey, false),
            ],
            data: instruction::DistributeFeesProcessFrame {}.data(),
        })
    } else {
        None
    };

    Ok(AutomationResponse {
        dynamic_instruction,
        trigger: None,
    })
}
