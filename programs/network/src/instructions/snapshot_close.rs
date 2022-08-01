use {
    crate::state::*,
    anchor_lang::prelude::*,
    clockwork_scheduler::{response::TaskResponse, state::Queue},
};

#[derive(Accounts)]
pub struct SnapshotClose<'info> {
    #[account(seeds = [SEED_AUTHORITY], bump)]
    pub authority: Account<'info, Authority>,

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

pub fn handler(ctx: Context<SnapshotClose>) -> Result<TaskResponse> {
    // Get accounts
    let queue = &mut ctx.accounts.queue;
    let snapshot = &mut ctx.accounts.snapshot;

    msg!("Closing snapshot {} {:#?}", snapshot.id, snapshot.status);

    // If snapshot is not archived, then noop and try again on next invocation.
    if snapshot.status != SnapshotStatus::Archived {
        return Ok(TaskResponse::default());
    }

    // If this snapshot has no entries, then close immediately
    let snapshot_pubkey = snapshot.key().clone();
    let snapshot_id = snapshot.id.clone();
    if snapshot.node_count == 0 {
        let snapshot_lamports = snapshot.to_account_info().lamports();
        **snapshot.to_account_info().lamports.borrow_mut() = 0;
        **queue.to_account_info().lamports.borrow_mut() = queue
            .to_account_info()
            .lamports()
            .checked_add(snapshot_lamports)
            .unwrap();
    } else {
        // Otherwise, set the status to closing
        snapshot.status = SnapshotStatus::Closing;
    }

    // Use dynamic accounts to run the next invocation with the next snapshot

    let next_snapshot_pubkey = Snapshot::pubkey(snapshot_id.checked_add(1).unwrap());
    Ok(TaskResponse {
        dynamic_accounts: Some(
            ctx.accounts
                .to_account_metas(None)
                .iter()
                .map(|acc| match acc.pubkey {
                    _ if acc.pubkey == snapshot_pubkey => next_snapshot_pubkey,
                    _ => acc.pubkey,
                })
                .collect(),
        ),
    })
}
