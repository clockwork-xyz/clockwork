use clockwork_utils::automation::{anchor_sighash, AccountMetaData, InstructionData, AutomationResponse};

use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct DeleteSnapshotProcessSnapshot<'info> {
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
        address = config.epoch_automation
    )]
    pub automation: Signer<'info>,
}

pub fn handler(ctx: Context<DeleteSnapshotProcessSnapshot>) -> Result<AutomationResponse> {
    // Get accounts
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;
    let automation = &mut ctx.accounts.automation;

    // If this snapshot has no entries, then close immediately
    if snapshot.total_frames.eq(&0) {
        let snapshot_lamports = snapshot.to_account_info().lamports();
        **snapshot.to_account_info().lamports.borrow_mut() = 0;
        **automation.to_account_info().lamports.borrow_mut() = automation
            .to_account_info()
            .lamports()
            .checked_add(snapshot_lamports)
            .unwrap();
    }

    // Build next instruction the automation.
    let dynamic_instruction = if snapshot.total_frames.gt(&0) {
        // There are frames in this snapshot. Delete them.
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::readonly(config.key(), false),
                AccountMetaData::readonly(registry.key(), false),
                AccountMetaData::mutable(snapshot.key(), false),
                AccountMetaData::mutable(SnapshotFrame::pubkey(snapshot.key(), 0), false),
                AccountMetaData::mutable(automation.key(), true),
            ],
            data: anchor_sighash("delete_snapshot_process_frame").to_vec(),
        })
    } else {
        // This snaphot has no frames. We are done!
        None
    };

    Ok(AutomationResponse { dynamic_instruction, trigger: None })
}
