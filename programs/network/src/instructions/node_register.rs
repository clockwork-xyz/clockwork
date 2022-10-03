use {
    crate::objects::*,
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

    #[account(mut, constraint = authority.key() != worker.key())]
    pub authority: Signer<'info>,

    #[account(address = Config::pubkey())]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init,
        address = SnapshotEntry::pubkey(snapshot.key(), snapshot.node_count),
        payer = authority,
        space = 8 + size_of::<SnapshotEntry>(),
    )]
    pub entry: Account<'info, SnapshotEntry>,

    #[account(address = config.mint)]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        address = Node::pubkey(registry.node_count),
        payer = authority,
        space = 8 + size_of::<Node>(),
    )]
    pub node: Account<'info, Node>,

    #[account(
        mut, 
        address = Registry::pubkey(),
        constraint = !registry.is_locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,

    #[account(
        mut,
        address = snapshot.pubkey(),
        constraint = snapshot.status == SnapshotStatus::Current
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(
        init,
        payer = authority,
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
    let authority = &mut ctx.accounts.authority;
    let node = &mut ctx.accounts.node;
    let registry = &mut ctx.accounts.registry;
    let stake = &mut ctx.accounts.stake;
    let worker = &ctx.accounts.worker;

    // Add node to the registry
    registry.new_node(authority, node, stake, worker)?;

    Ok(())
}
