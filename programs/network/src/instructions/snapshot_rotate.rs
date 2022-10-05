use {crate::objects::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct SnapshotRotate<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            current_snapshot.id.to_be_bytes().as_ref()
        ],
        bump,
        constraint = current_snapshot.status == SnapshotStatus::Current
    )]
    pub current_snapshot: Account<'info, Snapshot>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            next_snapshot.id.to_be_bytes().as_ref()
        ],
        bump,
        constraint = current_snapshot.id.checked_add(1).unwrap().eq(&next_snapshot.id)
    )]
    pub next_snapshot: Account<'info, Snapshot>,

    #[account(mut, seeds = [SEED_REGISTRY], bump)]
    pub registry: Account<'info, Registry>,

    #[account(address = config.automation_authority)]
    pub signer: Signer<'info>,
}

pub fn handler(ctx: Context<SnapshotRotate>) -> Result<()> {
    // Get accounts
    let current_snapshot = &mut ctx.accounts.current_snapshot;
    let next_snapshot = &mut ctx.accounts.next_snapshot;
    let registry = &mut ctx.accounts.registry;

    // Rotate the snapshot
    registry.rotate_snapshot(Some(current_snapshot), next_snapshot)?;

    Ok(())
}
