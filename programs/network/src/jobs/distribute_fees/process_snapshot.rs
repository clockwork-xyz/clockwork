use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
use clockwork_utils::thread::ThreadResponse;

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

    #[account(address = config.epoch_thread)]
    pub thread: Signer<'info>,
}

pub fn handler(ctx: Context<DistributeFeesProcessSnapshot>) -> Result<ThreadResponse> {
    let config = &ctx.accounts.config;
    let registry = &mut ctx.accounts.registry;
    let snapshot = &ctx.accounts.snapshot;
    let thread = &ctx.accounts.thread;

    Ok(ThreadResponse {
        dynamic_instruction: if snapshot.total_frames.gt(&0) {
            Some(
                Instruction {
                    program_id: crate::ID,
                    accounts: crate::accounts::DistributeFeesProcessFrame {
                        config: config.key(),
                        fee: Fee::pubkey(Worker::pubkey(0)),
                        registry: registry.key(),
                        snapshot: snapshot.key(),
                        snapshot_frame: SnapshotFrame::pubkey(snapshot.key(), 0),
                        thread: thread.key(),
                        worker: Worker::pubkey(0),
                    }
                    .to_account_metas(Some(true)),
                    data: crate::instruction::DistributeFeesProcessFrame {}.data(),
                }
                .into(),
            )
        } else {
            None
        },
        close_to: None,
        trigger: None,
    })
}
