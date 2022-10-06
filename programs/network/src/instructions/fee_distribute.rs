use {
    crate::objects::*, anchor_lang::prelude::*, anchor_spl::token::TokenAccount,
    clockwork_utils::CrankResponse,
};

#[derive(Accounts)]
pub struct FeeDistribute<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_DELEGATION,
            delegation.worker.as_ref(),
            delegation.id.to_be_bytes().as_ref(),
        ],
        bump,
        constraint = delegation.id.eq(&snapshot_entry.id),
        has_one = worker,
    )]
    pub delegation: Account<'info, Delegation>,

    #[account(
        address = epoch.pubkey(),
        constraint = registry.current_epoch_id.eq(&epoch.id),
    )]
    pub epoch: Account<'info, Epoch>,

    #[account(
        mut,
        seeds = [
            SEED_FEE,
            fee.worker.as_ref(),
        ],
        bump,
        has_one = worker,
    )]
    pub fee: Account<'info, Fee>,

    #[account(address = config.authorized_queue)]
    pub queue: Signer<'info>,

    #[account(address = Registry::pubkey())]
    pub registry: Account<'info, Registry>,

    #[account(
        address = snapshot.pubkey(),
        has_one = epoch,
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(
        address = snapshot_frame.pubkey(),
        has_one = snapshot,
        has_one = worker,
    )]
    pub snapshot_frame: Account<'info, SnapshotFrame>,

    #[account(
        address = snapshot_entry.pubkey(),
        has_one = snapshot_frame,
    )]
    pub snapshot_entry: Account<'info, SnapshotEntry>,

    #[account()]
    pub worker: Account<'info, Worker>,

    #[account(
        associated_token::authority = worker,
        associated_token::mint = config.mint,
    )]
    pub worker_tokens: Account<'info, TokenAccount>,
}

pub fn handler(ctx: Context<FeeDistribute>) -> Result<CrankResponse> {
    // TODO Distribute fees from the Fee accounts to the Delegation accounts according to the weight of the SnapshotEntry.

    Ok(CrankResponse {
        next_instruction: None,
    })
}
