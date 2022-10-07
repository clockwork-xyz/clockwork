use anchor_spl::associated_token::get_associated_token_address;
use clockwork_utils::{anchor_sighash, AccountMetaData, CrankResponse, InstructionData};

use {crate::objects::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct WorkerYieldFees<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = epoch.pubkey(),
        constraint = registry.current_epoch_id.eq(&epoch.id),
    )]
    pub epoch: Account<'info, Epoch>,

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

    #[account(address = config.authorized_queue)]
    pub queue: Signer<'info>,

    #[account(address = Registry::pubkey())]
    pub registry: Account<'info, Registry>,

    #[account(
        address = snapshot.pubkey(),
        has_one = epoch,
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(
        address = snapshot_frame.pubkey(),
        has_one = snapshot,
        has_one = worker,
    )]
    pub snapshot_frame: Account<'info, SnapshotFrame>,

    #[account(mut)]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<WorkerYieldFees>) -> Result<CrankResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let epoch = &ctx.accounts.epoch;
    let fee = &mut ctx.accounts.fee;
    let queue = &ctx.accounts.queue;
    let registry = &ctx.accounts.registry;
    let snapshot = &ctx.accounts.snapshot;
    let snapshot_frame = &ctx.accounts.snapshot_frame;
    let worker = &ctx.accounts.worker;

    // TODO Record the distributable fee balance.
    //
    //
    // fee.distributable_balance = fee.collected_balance;

    // TODO Build next instruction for the queue.
    let next_instruction = if snapshot_frame.total_entries.gt(&0) {
        // This snapshot frame has entries. Distribute fees to the delegations associated with the entries.
        let delegation_pubkey = Delegation::pubkey(worker.key(), 0);
        let snapshot_entry_pubkey = SnapshotEntry::pubkey(snapshot_frame.key(), 0);
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new(delegation_pubkey, false),
                AccountMetaData::new_readonly(epoch.key(), false),
                AccountMetaData::new(fee.key(), false),
                AccountMetaData::new_readonly(queue.key(), true),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(snapshot.key(), false),
                AccountMetaData::new_readonly(snapshot_frame.key(), false),
                AccountMetaData::new_readonly(snapshot_entry_pubkey.key(), false),
                AccountMetaData::new_readonly(worker.key(), false),
                AccountMetaData::new_readonly(
                    get_associated_token_address(&worker.key(), &config.mint),
                    false,
                ),
            ],
            data: anchor_sighash("fee_distribute").to_vec(),
        })
    } else if snapshot_frame
        .id
        .checked_add(1)
        .unwrap()
        .lt(&snapshot.total_frames)
    {
        // TODO If there are other frames, move on to next frame.
        None
    } else {
        // TODO If there are no other frames, move on to the next job!
        None
    };

    Ok(CrankResponse { next_instruction })
}
