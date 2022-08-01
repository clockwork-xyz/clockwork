use clockwork_scheduler::state::Queue;

use {crate::state::*, anchor_lang::prelude::*, clockwork_scheduler::response::TaskResponse};

#[derive(Accounts)]
pub struct SnapshotRotate<'info> {
    #[account(seeds = [SEED_AUTHORITY], bump)]
    pub authority: Account<'info, Authority>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            registry.snapshot_count.checked_sub(1).unwrap().to_be_bytes().as_ref()
        ],
        bump,
    )]
    pub current_snapshot: Account<'info, Snapshot>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            registry.snapshot_count.to_be_bytes().as_ref()
        ],
        bump,
    )]
    pub next_snapshot: Account<'info, Snapshot>,

    #[account(constraint = queue.authority == authority.key())]
    pub queue: Account<'info, Queue>,

    #[account(mut, seeds = [SEED_REGISTRY], bump)]
    pub registry: Account<'info, Registry>,
}

pub fn handler(ctx: Context<SnapshotRotate>) -> Result<TaskResponse> {
    // Get accounts
    let current_snapshot = &mut ctx.accounts.current_snapshot;
    let next_snapshot = &mut ctx.accounts.next_snapshot;
    let registry = &mut ctx.accounts.registry;

    // Rotate the snapshot
    let res = registry.rotate_snapshot(Some(current_snapshot), next_snapshot);
    if res.is_err() {
        // Don't return the error from here
        msg!("Snapshot rotation failed: {:?}", res.err())
    }

    // Use dynamic accounts to run the next invocation with the new current snapshot
    let snapshot_pubkey = current_snapshot.key();
    let next_snapshot_pubkey = next_snapshot.key();
    let next_next_snapshot_pubkey = Snapshot::pubkey(next_snapshot.id.checked_add(1).unwrap());
    Ok(TaskResponse {
        dynamic_accounts: Some(
            ctx.accounts
                .to_account_metas(None)
                .iter()
                .map(|acc| match acc.pubkey {
                    _ if acc.pubkey == snapshot_pubkey => next_snapshot_pubkey,
                    _ if acc.pubkey == next_snapshot_pubkey => next_next_snapshot_pubkey,
                    _ => acc.pubkey,
                })
                .collect(),
        ),
    })
}
