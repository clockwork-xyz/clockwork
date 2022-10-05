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
            snapshot.total_workers.to_be_bytes().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<SnapshotEntry>(),
    )]
    pub entry: Account<'info, SnapshotEntry>,

    #[account(
        address = node.pubkey(),
        constraint = node.id == snapshot.total_workers @ ClockworkError::InvalidNode
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
            snapshot.epoch.as_ref(),
        ],
        bump,
        constraint = snapshot.status == SnapshotStatus::Capturing @ ClockworkError::SnapshotNotInProgress,
        constraint = snapshot.total_workers < registry.total_workers,
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
