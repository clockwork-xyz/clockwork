use clockwork_utils::automation::{  AutomationResponse, InstructionBuilder};
use anchor_lang::{prelude::*, InstructionData};

use crate::{state::*, instruction};

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
        Some(
            InstructionBuilder::new(crate::ID)
            .readonly_account(config.key())
            .readonly_account(registry.key())
            .mutable_account(snapshot.key())
            .mutable_account(SnapshotFrame::pubkey(snapshot.key(), 0))
            .signer(automation.key())
            .data(instruction::DeleteSnapshotProcessFrame{}.data())
            .build()
        )
    } else {
        // This snaphot has no frames. We are done!
        None
    };

    Ok(AutomationResponse { dynamic_instruction, trigger: None })
}
