use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::instruction::Instruction},
    clockwork_queue_program::state::{CrankResponse, Queue},
};

#[derive(Accounts)]
pub struct EntryClose<'info> {
    #[account(seeds = [SEED_AUTHORITY], bump)]
    pub authority: Account<'info, Authority>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT_ENTRY,
            entry.snapshot.as_ref(),
            entry.id.to_be_bytes().as_ref()
        ],
        bump,
        has_one = snapshot,
    )]
    pub entry: Account<'info, SnapshotEntry>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            snapshot.id.to_be_bytes().as_ref()
        ],
        bump,
        constraint = snapshot.status == SnapshotStatus::Closing,
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(mut, has_one = authority, constraint = snapshot_queue.id.eq("snapshot"))]
    pub snapshot_queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<EntryClose>) -> Result<CrankResponse> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let entry = &mut ctx.accounts.entry;
    let snapshot = &mut ctx.accounts.snapshot;
    let snapshot_queue = &mut ctx.accounts.snapshot_queue;

    // If snapshot is not closing, then noop and try again on next invocation.
    if snapshot.status != SnapshotStatus::Closing {
        return Ok(CrankResponse::default());
    }

    // Close the entry account.
    let entry_id = entry.id.clone();
    let entry_lamports = entry.to_account_info().lamports();
    **entry.to_account_info().lamports.borrow_mut() = 0;
    **snapshot_queue.to_account_info().lamports.borrow_mut() = snapshot_queue
        .to_account_info()
        .lamports()
        .checked_add(entry_lamports)
        .unwrap();

    // If this is the last entry of the snapshot, then also close the snapshot account.
    let snapshot_pubkey = snapshot.key().clone();
    let snapshot_node_count = snapshot.node_count.clone();
    if entry_id == snapshot_node_count.checked_sub(1).unwrap() {
        let snapshot_lamports = snapshot.to_account_info().lamports();
        **snapshot.to_account_info().lamports.borrow_mut() = 0;
        **snapshot_queue.to_account_info().lamports.borrow_mut() = snapshot_queue
            .to_account_info()
            .lamports()
            .checked_add(snapshot_lamports)
            .unwrap();
    }

    // Use dynamic accounts to run with the next snapshot on the next invocation
    let next_instruction = if entry_id < snapshot.node_count.checked_sub(1).unwrap() {
        let next_entry_pubkey =
            SnapshotEntry::pubkey(snapshot_pubkey, entry.id.checked_add(1).unwrap());
        Some(
            Instruction {
                program_id: crate::ID,
                accounts: vec![
                    AccountMeta::new_readonly(authority.key(), false),
                    AccountMeta::new(next_entry_pubkey, false),
                    AccountMeta::new(snapshot.key(), false),
                    AccountMeta::new(snapshot_queue.key(), false),
                ],
                data: clockwork_queue_program::utils::anchor_sighash("entry_close").into(),
            }
            .into(),
        )
    } else {
        None
    };

    Ok(CrankResponse { next_instruction })
}
