use {
    crate::{errors::CronosError, state::*},
    anchor_lang::{prelude::*, solana_program::system_program},
    anchor_spl::token::TokenAccount,
    cronos_scheduler::{responses::ExecResponse, state::Yogi},
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct SnapshotCapture<'info> {
    #[account(seeds = [SEED_AUTHORITY], bump, has_one = yogi)]
    pub authority: Box<Account<'info, Authority>>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT_ENTRY,
            snapshot.key().as_ref(),
            snapshot.entry_count.to_be_bytes().as_ref()
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<SnapshotEntry>()
    )]
    pub entry: Account<'info, SnapshotEntry>,

    #[account(
        seeds = [
            SEED_NODE,
            node.delegate.as_ref(),
        ],
        bump,
        constraint = node.id == snapshot.entry_count @ CronosError::InvalidNode
    )]
    pub node: Box<Account<'info, Node>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(signer, constraint = yogi.owner == authority.key())]
    pub yogi: Box<Account<'info, Yogi>>,

    #[account(seeds = [SEED_REGISTRY], bump)]
    pub registry: Box<Account<'info, Registry>>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            registry.snapshot_count.to_be_bytes().as_ref()
        ],
        bump,
        constraint = snapshot.status == SnapshotStatus::InProgress @ CronosError::SnapshotNotInProgress,
        constraint = snapshot.entry_count < registry.node_count,
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(
        associated_token::authority = node,
        associated_token::mint = config.mint,
    )]
    pub stake: Account<'info, TokenAccount>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SnapshotCapture>) -> Result<ExecResponse> {
    // Get accounts
    let entry = &mut ctx.accounts.entry;
    let node = &ctx.accounts.node;
    let stake = &ctx.accounts.stake;
    let snapshot = &mut ctx.accounts.snapshot;

    // Capture the snapshot entry
    snapshot.capture(entry, node, stake)?;

    // Use dynamic accounts to run the next invocation with the new current snapshot
    let entry_pubkey = entry.key();
    let snapshot_pubkey = snapshot.key();
    let next_snapshot_pubkey = Snapshot::pda(snapshot.id.checked_add(1).unwrap()).0;
    let next_entry_pubkey = SnapshotEntry::pda(next_snapshot_pubkey, entry.id).0;
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
