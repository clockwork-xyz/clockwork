use anchor_lang::prelude::*;
use clockwork_utils::automation::{
    anchor_sighash, AccountMetaData, InstructionData, AutomationResponse,
};

use crate::state::*;

// DONE Payout yield.
//      Transfer lamports collected by Fee accounts to Delegation accounts based on the stake balance distributions of the current Epoch's SnapshotEntries.

// DONE Process unstake requests.
//      For each "unstake request" transfer tokens from the Worker stake account to the Delegation authority's token account.
//      Decrement the Delegation's stake balance by the amount unstaked.

// DONE Lock delegated stakes.
//      Transfer tokens from the Delegation's stake account to the Worker's stake account.
//      Increment the Delegation's stake balance by the amount moved.

// DONE Take a snapshot.
//      Capture a snapshot (cumulative sum) of the total stake and broken-down delegation balances.
//      SnapshotFrames capture worker-level aggregate stake balances.
//      SnapshotEntries capture delegation-level individual stake balances.

// DONE Cutover from current epoch to new epoch.

#[derive(Accounts)]
pub struct DistributeFeesProcessSnapshot<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(seeds = [SEED_REGISTRY], bump)]
    pub registry: Account<'info, Registry>,

    #[account(
        address = snapshot.pubkey(),
        constraint = snapshot.id.eq(&registry.current_epoch)
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(address = config.epoch_automation)]
    pub automation: Signer<'info>,
}

pub fn handler(ctx: Context<DistributeFeesProcessSnapshot>) -> Result<AutomationResponse> {
    let config = &ctx.accounts.config;
    let registry = &mut ctx.accounts.registry;
    let snapshot = &ctx.accounts.snapshot;
    let automation = &ctx.accounts.automation;

    Ok(AutomationResponse {
        next_instruction: if snapshot.total_frames.gt(&0) {
            Some(InstructionData {
                program_id: crate::ID,
                accounts: vec![
                    AccountMetaData::new_readonly(config.key(), false),
                    AccountMetaData::new(Fee::pubkey(Worker::pubkey(0)), false),
                    AccountMetaData::new_readonly(registry.key(), false),
                    AccountMetaData::new_readonly(snapshot.key(), false),
                    AccountMetaData::new_readonly(SnapshotFrame::pubkey(snapshot.key(), 0), false),
                    AccountMetaData::new_readonly(automation.key(), true),
                    AccountMetaData::new(Worker::pubkey(0), false),
                ],
                data: anchor_sighash("distribute_fees_process_frame").to_vec(),
            })
        } else {
            None
        },
        trigger: None,
    })
}
