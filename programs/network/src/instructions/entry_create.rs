use {
    crate::{errors::ClockworkError, objects::*},
    anchor_lang::{prelude::*, solana_program::system_program},
    anchor_spl::token::TokenAccount,
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct EntryCreate<'info> {
    #[account(address = Config::pubkey())]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT_ENTRY,
            snapshot.key().as_ref(),
            snapshot.node_count.to_be_bytes().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<SnapshotEntry>(),
    )]
    pub entry: Account<'info, SnapshotEntry>,

    #[account(
        address = node.pubkey(),
        constraint = node.id == snapshot.node_count @ ClockworkError::InvalidNode
    )]
    pub node: Box<Account<'info, Node>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = Registry::pubkey())]
    pub registry: Box<Account<'info, Registry>>,

    #[account(address = config.automation_authority)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            snapshot.id.to_be_bytes().as_ref(),
        ],
        bump,
        constraint = snapshot.status == SnapshotStatus::InProgress @ ClockworkError::SnapshotNotInProgress,
        constraint = snapshot.node_count < registry.node_count,
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

pub fn handler(ctx: Context<EntryCreate>) -> Result<()> {
    // Get accounts
    let entry = &mut ctx.accounts.entry;
    let node = &ctx.accounts.node;
    let stake = &ctx.accounts.stake;
    let snapshot = &mut ctx.accounts.snapshot;

    // Capture the snapshot entry
    snapshot.capture(entry, node, stake)?;

    Ok(())
}
