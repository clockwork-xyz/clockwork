use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::instruction::Instruction},
    clockwork_crank::state::{CrankResponse, Queue, SEED_QUEUE},
};

#[derive(Accounts)]
pub struct SnapshotClose<'info> {
    #[account(seeds = [SEED_AUTHORITY], bump)]
    pub authority: Account<'info, Authority>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            snapshot.id.to_be_bytes().as_ref(),
        ],
        bump,
        constraint = snapshot.status == SnapshotStatus::Archived
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(
        signer, 
        seeds = [
            SEED_QUEUE, 
            authority.key().as_ref(), 
            "snapshot".as_bytes()
        ], 
        seeds::program = clockwork_crank::ID,
        bump,
        has_one = authority
    )]
    pub snapshot_queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<SnapshotClose>) -> Result<CrankResponse> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let snapshot = &mut ctx.accounts.snapshot;
    let snapshot_queue = &mut ctx.accounts.snapshot_queue;

    // If this snapshot has no entries, then close immediately
    if snapshot.node_count == 0 {
        let snapshot_lamports = snapshot.to_account_info().lamports();
        **snapshot.to_account_info().lamports.borrow_mut() = 0;
        **snapshot_queue.to_account_info().lamports.borrow_mut() = snapshot_queue
            .to_account_info()
            .lamports()
            .checked_add(snapshot_lamports)
            .unwrap();
    } else {
        // Otherwise, set the status to closing
        snapshot.status = SnapshotStatus::Closing;
    }

    // If there are entries to capture, build the next instruction
    let next_instruction = if snapshot.node_count > 0 {
        let entry_pubkey = SnapshotEntry::pubkey(snapshot.key(), 0);
        Some(
            Instruction {
                program_id: crate::ID,
                accounts: vec![
                    AccountMeta::new_readonly(authority.key(), false),
                    AccountMeta::new(entry_pubkey, false),
                    AccountMeta::new(snapshot.key(), false),
                    AccountMeta::new(snapshot_queue.key(), true),
                ],
                data: clockwork_crank::anchor::sighash("entry_close").into(),
            }.into()
        )
    } else {
        None
    };

    Ok(CrankResponse { next_instruction })
}
