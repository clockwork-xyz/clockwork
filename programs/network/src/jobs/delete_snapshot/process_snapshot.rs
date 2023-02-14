use clockwork_utils::thread::ThreadResponse;
use anchor_lang::{prelude::*, InstructionData, solana_program::instruction::Instruction};

use crate::state::*;

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
        address = config.epoch_thread
    )]
    pub thread: Signer<'info>,
}

pub fn handler(ctx: Context<DeleteSnapshotProcessSnapshot>) -> Result<ThreadResponse> {
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
    let dynamic_instruction = if snapshot.total_frames.gt(&0) {
        // There are frames in this snapshot. Delete them.
        Some(
            Instruction {
                program_id: crate::ID,
                accounts: crate::accounts::DeleteSnapshotProcessFrame {
                    config: config.key(),
                    registry: registry.key(),
                    snapshot: snapshot.key(),
                    snapshot_frame: SnapshotFrame::pubkey(snapshot.key(), 0),
                    thread: thread.key(),
                }.to_account_metas(Some(true)),
                data: crate::instruction::DeleteSnapshotProcessFrame{}.data()
            }.into()
        )
    } else {
        // This snaphot has no frames. We are done!
        None
    };

    Ok(ThreadResponse { dynamic_instruction, close_to:None, trigger: None })
}
