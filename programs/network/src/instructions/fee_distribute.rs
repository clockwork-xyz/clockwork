use {
    crate::objects::*,
    anchor_lang::prelude::*,
    clockwork_utils::{anchor_sighash, AccountMetaData, CrankResponse, InstructionData},
};

#[derive(Accounts)]
pub struct FeeDistribute<'info> {
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

    #[account(
        address = snapshot_entry.pubkey(),
        has_one = snapshot_frame,
    )]
    pub snapshot_entry: Account<'info, SnapshotEntry>,

    #[account()]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<FeeDistribute>) -> Result<CrankResponse> {
    // Get accounts
    let config = &ctx.accounts.config;
    let delegation = &mut ctx.accounts.delegation;
    let epoch = &ctx.accounts.epoch;
    let fee = &mut ctx.accounts.fee;
    let queue = &ctx.accounts.queue;
    let registry = &ctx.accounts.registry;
    let snapshot = &ctx.accounts.snapshot;
    let snapshot_entry = &ctx.accounts.snapshot_entry;
    let snapshot_frame = &ctx.accounts.snapshot_frame;
    let worker = &ctx.accounts.worker;

    // Calculate the yield balance of this particular delegation, based on the weight of its stake with this worker.
    let yield_balance = fee
        .distributable_balance
        .checked_mul(snapshot_entry.stake_amount)
        .unwrap()
        .checked_div(snapshot_frame.stake_amount)
        .unwrap();

    // Transfer yield to the worker.
    **fee.to_account_info().try_borrow_mut_lamports()? = fee
        .to_account_info()
        .lamports()
        .checked_sub(yield_balance)
        .unwrap();
    **delegation.to_account_info().try_borrow_mut_lamports()? = worker
        .to_account_info()
        .lamports()
        .checked_add(yield_balance)
        .unwrap();

    // Increment the delegation's yield balance.
    delegation.yield_balance = delegation.yield_balance.checked_add(yield_balance).unwrap();

    // Build the next instruction for the queue.
    let next_instruction = if snapshot_entry
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
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new(next_delegation_pubkey, false),
                AccountMetaData::new_readonly(epoch.key(), false),
                AccountMetaData::new(fee.key(), false),
                AccountMetaData::new_readonly(queue.key(), true),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(snapshot.key(), false),
                AccountMetaData::new_readonly(snapshot_frame.key(), false),
                AccountMetaData::new_readonly(next_snapshot_entry_pubkey, false),
                AccountMetaData::new_readonly(worker.key(), false),
            ],
            data: anchor_sighash("fee_distribute").to_vec(),
        })
    } else if snapshot_frame
        .id
        .checked_add(1)
        .unwrap()
        .lt(&snapshot.total_frames)
    {
        // This frame has no more entries. Move on to the next worker.
        let next_worker_pubkey = Worker::pubkey(worker.id.checked_add(1).unwrap());
        let next_snapshot_frame_pubkey =
            SnapshotFrame::pubkey(snapshot_frame.id.checked_add(1).unwrap(), snapshot.key());
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(epoch.key(), false),
                AccountMetaData::new(fee.key(), false),
                AccountMetaData::new_readonly(queue.key(), true),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(snapshot.key(), false),
                AccountMetaData::new_readonly(next_snapshot_frame_pubkey, false),
                AccountMetaData::new_readonly(next_worker_pubkey, false),
            ],
            data: anchor_sighash("worker_distribute_fees").to_vec(),
        })
    } else {
        // TODO If this frame has no more entires and it is the last frame, move on to the next job.

        None
    };

    Ok(CrankResponse { next_instruction })
}
