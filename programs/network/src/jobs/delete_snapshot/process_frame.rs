use anchor_lang::{prelude::*, InstructionData};
use clockwork_utils::automation::{ AutomationResponse, InstructionBuilder};

use crate::{instruction, state::*};

#[derive(Accounts)]
pub struct DeleteSnapshotProcessFrame<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = Registry::pubkey(),
        constraint = !registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            snapshot.id.to_be_bytes().as_ref(),
        ],
        bump,
        constraint = snapshot.id.lt(&registry.current_epoch)
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT_FRAME,
            snapshot_frame.snapshot.as_ref(),
            snapshot_frame.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = snapshot,
    )]
    pub snapshot_frame: Account<'info, SnapshotFrame>,

    #[account(
        mut, 
        address = config.epoch_automation
    )]
    pub automation: Signer<'info>,
}

pub fn handler(ctx: Context<DeleteSnapshotProcessFrame>) -> Result<AutomationResponse> {
    // Get accounts
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;
    let snapshot_frame = &mut ctx.accounts.snapshot_frame;
    let automation = &mut ctx.accounts.automation;

    // If this frame has no entries, then close the frame account.
    if snapshot_frame.total_entries.eq(&0) {
        let snapshot_frame_lamports = snapshot_frame.to_account_info().lamports();
        **snapshot_frame.to_account_info().lamports.borrow_mut() = 0;
        **automation.to_account_info().lamports.borrow_mut() = automation
            .to_account_info()
            .lamports()
            .checked_add(snapshot_frame_lamports)
            .unwrap();


        // If this is also the last frame in the snapshot, then close the snapshot account.
        if snapshot_frame.id.checked_add(1).unwrap().eq(&snapshot.total_frames) {
            let snapshot_lamports = snapshot.to_account_info().lamports();
            **snapshot.to_account_info().lamports.borrow_mut() = 0;
            **automation.to_account_info().lamports.borrow_mut() = automation
                .to_account_info()
                .lamports()
                .checked_add(snapshot_lamports)
                .unwrap();
        }
    }

    // Build the next instruction.
    let dynamic_instruction = if snapshot_frame.total_entries.gt(&0) {
        // This frame has entries. Delete the entries.
        Some(
            InstructionBuilder::new(crate::ID)
            .readonly_account(config.key())
            .readonly_account(registry.key())
            .mutable_account(snapshot.key())
            .mutable_account(SnapshotEntry::pubkey(snapshot_frame.key(), 0))
            .mutable_account(snapshot_frame.key())
            .signer(automation.key())
            .data(instruction::DeleteSnapshotProcessEntry{}.data())
            .build()
        )
    } else if snapshot_frame.id.checked_add(1).unwrap().lt(&snapshot.total_frames) {
        // There are no more entries in this frame. Move on to the next frame.
        Some(
            InstructionBuilder::new(crate::ID)
            .readonly_account(config.key())
            .readonly_account(registry.key())
            .mutable_account(snapshot.key())
            .mutable_account(SnapshotFrame::pubkey(snapshot.key(), snapshot_frame.id.checked_add(1).unwrap()))
            .signer(automation.key())
            .data(instruction::DeleteSnapshotProcessFrame{}.data())
            .build()
        )
    } else {
        // This frame has no entries, and it was the last frame. We are done!
        None
    };

    Ok( AutomationResponse { dynamic_instruction, ..AutomationResponse::default() } )
}
