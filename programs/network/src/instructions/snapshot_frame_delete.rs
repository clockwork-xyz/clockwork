use clockwork_utils::{InstructionData, AccountMetaData, anchor_sighash};

use {crate::state::*, anchor_lang::prelude::*, clockwork_utils::ThreadResponse};

#[derive(Accounts)]
pub struct SnapshotFrameDelete<'info> {
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
        address = config.epoch_thread
    )]
    pub thread: Signer<'info>,
}

pub fn handler(ctx: Context<SnapshotFrameDelete>) -> Result<ThreadResponse> {
    // Get accounts
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;
    let snapshot_frame = &mut ctx.accounts.snapshot_frame;
    let thread = &mut ctx.accounts.thread;

    // If this frame has no entries, then close the frame account.
    if snapshot_frame.total_entries.eq(&0) {
        let snapshot_frame_lamports = snapshot_frame.to_account_info().lamports();
        **snapshot_frame.to_account_info().lamports.borrow_mut() = 0;
        **thread.to_account_info().lamports.borrow_mut() = thread
            .to_account_info()
            .lamports()
            .checked_add(snapshot_frame_lamports)
            .unwrap();


        // If this is also the last frame in the snapshot, then close the snapshot account.
        if snapshot_frame.id.checked_add(1).unwrap().eq(&snapshot.total_frames) {
            let snapshot_lamports = snapshot.to_account_info().lamports();
            **snapshot.to_account_info().lamports.borrow_mut() = 0;
            **thread.to_account_info().lamports.borrow_mut() = thread
                .to_account_info()
                .lamports()
                .checked_add(snapshot_lamports)
                .unwrap();
        }
    }

    // Build the next instruction.
    let next_instruction = if snapshot_frame.total_entries.gt(&0) {
        // This frame has entries. Delete the entries.
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new(snapshot.key(), false),
                AccountMetaData::new(SnapshotEntry::pubkey(snapshot_frame.key(), 0), false),
                AccountMetaData::new(snapshot_frame.key(), false),
                AccountMetaData::new(thread.key(), true),
            ],
            data: anchor_sighash("snapshot_entry_delete").to_vec(),
        })
    } else if snapshot_frame.id.checked_add(1).unwrap().lt(&snapshot.total_frames) {
        // There are no more entries in this frame. Move on to the next frame.
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new(snapshot.key(), false),
                AccountMetaData::new(SnapshotFrame::pubkey(snapshot.key(), snapshot_frame.id.checked_add(1).unwrap()), false),
                AccountMetaData::new(thread.key(), true),
            ],
            data: anchor_sighash("snapshot_frame_delete").to_vec(),
        })
    } else {
        // This frame has no entries, and it was the last frame. We are done!
        None
    };

    Ok( ThreadResponse { next_instruction, ..ThreadResponse::default() } )
}
