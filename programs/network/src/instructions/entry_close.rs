use {
    crate::state::*,
    anchor_lang::prelude::*,
    cronos_scheduler::{responses::ExecResponse, state::Queue},
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
        close = queue,
    )]
    pub entry: Account<'info, SnapshotEntry>,

    #[account(mut, has_one = authority, constraint = queue.id == 1)]
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

pub fn handler(ctx: Context<EntryClose>) -> Result<ExecResponse> {
    // Get accounts
    let entry = &mut ctx.accounts.entry;
    let snapshot = &mut ctx.accounts.snapshot;
    let queue = &mut ctx.accounts.queue;

    msg!("Closing entry {} {:#?}", entry.id, snapshot.status);

    // If snapshot is not closing, then noop and try again on next invocation.
    if snapshot.status != SnapshotStatus::Closing {
        return Ok(ExecResponse::default());
    }

    // If this is the last entry of the snapshot, then also close the snapshot account.
    if entry.id == snapshot.node_count.checked_sub(1).unwrap() {
        let snapshot_lamports = snapshot.to_account_info().try_lamports()?;
        **snapshot.to_account_info().try_borrow_mut_lamports()? = snapshot
            .to_account_info()
            .lamports()
            .checked_sub(snapshot_lamports)
            .unwrap();
        **queue.to_account_info().try_borrow_mut_lamports()? = queue
            .to_account_info()
            .lamports()
            .checked_add(snapshot_lamports)
            .unwrap();
    }

    // Use dynamic accounts to run with the next snapshot on the next invocation
    let entry_pubkey = entry.key();
    let snapshot_pubkey = snapshot.key();
    let next_snapshot_pubkey = Snapshot::pubkey(snapshot.id.checked_add(1).unwrap());
    let next_entry_pubkey = SnapshotEntry::pubkey(next_snapshot_pubkey, entry.id);
    Ok(ExecResponse {
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
