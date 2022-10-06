use {
    crate::objects::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    anchor_spl::{associated_token::get_associated_token_address, token::TokenAccount},
    clockwork_utils::CrankResponse,
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct SnapshotFrameCreate<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = current_epoch.pubkey(),
        constraint = epoch.current
    )]
    pub current_epoch: Account<'info, Epoch>,

    #[account(
        address = epoch.pubkey(),
        constraint = current_epoch.id.checked_add(1).unwrap().eq(&epoch.id),
    )]
    pub epoch: Account<'info, Epoch>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        address = Registry::pubkey(),
        constraint = registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            snapshot.epoch.as_ref(),
        ],
        bump,
        has_one = epoch,
        constraint = snapshot.total_workers < registry.total_workers,
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account()]
    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT_FRAME,
            snapshot.key().as_ref(),
            snapshot.total_workers.to_be_bytes().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<SnapshotFrame>(),
    )]
    pub snapshot_frame: Account<'info, SnapshotFrame>,

    #[account(
        associated_token::authority = worker,
        associated_token::mint = config.mint,
    )]
    pub stake: Account<'info, TokenAccount>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(
        address = worker.pubkey(),
        constraint = worker.id.eq(&snapshot.total_workers),
    )]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<SnapshotFrameCreate>) -> Result<CrankResponse> {
    // Get accounts.
    let registry = &ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;
    let snapshot_frame = &mut ctx.accounts.snapshot_frame;
    let stake = &ctx.accounts.stake;
    let worker = &ctx.accounts.worker;

    // Initialize snapshot frame account.
    snapshot_frame.init(
        snapshot.total_workers,
        snapshot.key(),
        stake.amount,
        snapshot.total_stake,
        worker.key(),
    )?;

    // Update snapshot total workers.
    snapshot.total_stake = snapshot.total_stake.checked_add(stake.amount).unwrap();
    snapshot.total_workers = snapshot.total_workers.checked_add(1).unwrap();

    // We just created the frame...
    // Now go through each delegation on this worker.
    // If there are no delegations on the worker, move to the worker.
    // If there are no more workers, then the snapshot is done!

    // TODO Return next instruction.
    Ok(CrankResponse {
        next_instruction: None,
    })
}
