use {
    crate::state::*,
    anchor_lang::{
        prelude::*,
        solana_program::{system_program, sysvar},
    },
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{Mint, Token, TokenAccount},
    },
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct NodeRegister<'info> {
    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT_ENTRY,
            snapshot.key().as_ref(),
            snapshot.node_count.to_be_bytes().as_ref(),
        ],
        bump,
        payer = signer,
        space = 8 + size_of::<SnapshotEntry>(),
    )]
    pub entry: Account<'info, SnapshotEntry>,

    #[account(address = config.mint)]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [
            SEED_NODE,
            registry.node_count.to_be_bytes().as_ref()
        ],
        bump,
        payer = signer,
        space = 8 + size_of::<Node>(),
    )]
    pub node: Account<'info, Node>,

    #[account(
        mut, 
        seeds = [SEED_REGISTRY], 
        bump,
        constraint = !registry.is_locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,

    #[account(mut, constraint = signer.key() != worker.key())]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            snapshot.id.to_be_bytes().as_ref(),
        ],
        bump,
        constraint = snapshot.status == SnapshotStatus::Current
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(
        init,
        payer = signer,
        associated_token::authority = node,
        associated_token::mint = mint,
    )]
    pub stake: Account<'info, TokenAccount>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account()]
    pub worker: Signer<'info>,
}

pub fn handler(ctx: Context<NodeRegister>) -> Result<()> {
    // Get accounts
    let node = &mut ctx.accounts.node;
    let registry = &mut ctx.accounts.registry;
    let signer = &mut ctx.accounts.signer;
    let stake = &mut ctx.accounts.stake;
    let worker = &ctx.accounts.worker;

    // Add node to the registry
    registry.new_node(signer, node, stake, worker)?;

    Ok(())
}
