use {
    crate::state::*,
    anchor_lang::prelude::*,
    cronos_scheduler::{responses::TaskResponse, state::Queue},
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

    #[account(mut, has_one = authority, constraint = queue.name.eq("cleanup"))]
    pub queue: Account<'info, Queue>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            snapshot.id.to_be_bytes().as_ref()
        ],
        bump,
    )]
    pub snapshot: Account<'info, Snapshot>,
}

pub fn handler(ctx: Context<EntryClose>) -> Result<TaskResponse> {
    // Get accounts
    let entry = &mut ctx.accounts.entry;
    let queue = &mut ctx.accounts.queue;
    let snapshot = &mut ctx.accounts.snapshot;

    msg!(
        "Closing entry {} of snapshot {} in status {:#?}",
        entry.id,
        snapshot.id,
        snapshot.status
    );

    // If snapshot is not closing, then noop and try again on next invocation.
    if snapshot.status != SnapshotStatus::Closing {
        return Ok(TaskResponse::default());
    }

    // Close the entry account.
    let entry_id = entry.id.clone();
    let entry_pubkey = entry.key().clone();
    let entry_lamports = entry.to_account_info().lamports();
    **entry.to_account_info().lamports.borrow_mut() = 0;
    **queue.to_account_info().lamports.borrow_mut() = queue
        .to_account_info()
        .lamports()
        .checked_add(entry_lamports)
        .unwrap();

    // If this is the last entry of the snapshot, then also close the snapshot account.
    let snapshot_id = snapshot.id.clone();
    let snapshot_pubkey = snapshot.key().clone();
    let snapshot_node_count = snapshot.node_count.clone();
    if entry_id == snapshot_node_count.checked_sub(1).unwrap() {
        let snapshot_lamports = snapshot.to_account_info().lamports();
        **snapshot.to_account_info().lamports.borrow_mut() = 0;
        **queue.to_account_info().lamports.borrow_mut() = queue
            .to_account_info()
            .lamports()
            .checked_add(snapshot_lamports)
            .unwrap();
    }

    // Use dynamic accounts to run with the next snapshot on the next invocation
    let next_snapshot_pubkey = Snapshot::pubkey(snapshot_id.checked_add(1).unwrap());
    let next_entry_pubkey = SnapshotEntry::pubkey(next_snapshot_pubkey, entry_id);
    Ok(TaskResponse {
        dynamic_accounts: Some(
            ctx.accounts
                .to_account_metas(None)
                .iter()
                .map(|acc| match acc.pubkey {
                    _ if acc.pubkey == entry_pubkey => next_entry_pubkey,
                    _ if acc.pubkey == snapshot_pubkey => next_snapshot_pubkey,
                    _ => acc.pubkey,
                })
                .collect(),
        ),
    })
}
