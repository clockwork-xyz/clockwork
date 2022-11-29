use clockwork_utils::{anchor_sighash, AccountMetaData, InstructionData};

use {crate::state::*, anchor_lang::prelude::*, clockwork_utils::ThreadResponse};

#[derive(Accounts)]
pub struct SnapshotDelete<'info> {
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
        address = config.epoch_thread
    )]
    pub thread: Signer<'info>,

}

pub fn handler(ctx: Context<SnapshotDelete>) -> Result<ThreadResponse> {
    // Get accounts
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;
    let thread = &mut ctx.accounts.thread;

    // If this snapshot has no entries, then close immediately
    if snapshot.total_frames.eq(&0) {
        let snapshot_lamports = snapshot.to_account_info().lamports();
        **snapshot.to_account_info().lamports.borrow_mut() = 0;
        **thread.to_account_info().lamports.borrow_mut() = thread
            .to_account_info()
            .lamports()
            .checked_add(snapshot_lamports)
            .unwrap();
    }

    // Build next instruction the thread.
    let next_instruction = if snapshot.total_frames.gt(&0) {
        // There are frames in this snapshot. Delete them.
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new(snapshot.key(), false),
                AccountMetaData::new(SnapshotFrame::pubkey(snapshot.key(), 0), false),
                AccountMetaData::new(thread.key(), true),
            ],
            data: anchor_sighash("snapshot_frame_delete").to_vec(),
        })
    } else {
        // This snaphot has no frames. We are done!
        None
    };

    Ok(ThreadResponse { next_instruction, ..ThreadResponse::default() })
}
