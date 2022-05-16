use {
    crate::state::*,
    anchor_lang::prelude::*,
    cronos_scheduler::{responses::ExecResponse, state::Manager},
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct SnapshotStart<'info> {
    #[account(seeds = [SEED_AUTHORITY], bump, has_one = manager)]
    pub authority: Box<Account<'info, Authority>>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(signer, constraint = manager.authority == authority.key())]
    pub manager: Box<Account<'info, Manager>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, seeds = [SEED_REGISTRY], bump)]
    pub registry: Account<'info, Registry>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT,
            registry.snapshot_count.to_be_bytes().as_ref()
        ],
        bump,
        space = 8 + size_of::<Snapshot>(),
        payer = payer
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SnapshotStart>) -> Result<ExecResponse> {
    // Get accounts
    let registry = &mut ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;

    // Start a new snapshot
    registry.new_snapshot(snapshot)?;

    // Use dynamic accounts to run the next invocation with the new current snapshot
    let snapshot_pubkey = snapshot.key();
    let next_snapshot_pubkey = Snapshot::pda(snapshot.id.checked_add(1).unwrap()).0;
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
