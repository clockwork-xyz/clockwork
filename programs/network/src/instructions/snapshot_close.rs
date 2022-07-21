use {
    crate::state::*,
    anchor_lang::prelude::*,
    cronos_scheduler::{responses::ExecResponse, state::Queue},
};

#[derive(Accounts)]
pub struct SnapshotClose<'info> {
    #[account(seeds = [SEED_AUTHORITY], bump)]
    pub authority: Account<'info, Authority>,

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

pub fn handler(ctx: Context<SnapshotClose>) -> Result<ExecResponse> {
    // Get accounts
    let queue = &mut ctx.accounts.queue;
    let snapshot = &mut ctx.accounts.snapshot;

    msg!("Closing snapshot {} {:#?}", snapshot.id, snapshot.status);

    // If snapshot is not archived, then noop and try again on next invocation.
    if snapshot.status != SnapshotStatus::Archived {
        return Ok(ExecResponse::default());
    }

    // If this snapshot has no entries, then close immediately
    if snapshot.node_count == 0 {
        let amount = snapshot.to_account_info().try_lamports()?;
        **snapshot.to_account_info().try_borrow_mut_lamports()? = snapshot
            .to_account_info()
            .lamports()
            .checked_sub(amount)
            .unwrap();
        **queue.to_account_info().try_borrow_mut_lamports()? = queue
            .to_account_info()
            .lamports()
            .checked_add(amount)
            .unwrap();
    } else {
        // Otherwise, set the status to closing
        snapshot.status = SnapshotStatus::Closing;
    }

    // Use dynamic accounts to run the next invocation with the next snapshot
    let snapshot_pubkey = snapshot.key();
    let next_snapshot_pubkey = Snapshot::pubkey(snapshot.id.checked_add(1).unwrap());
    Ok(ExecResponse {
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
