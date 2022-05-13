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
    cronos_scheduler::state::{Queue, Task},
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct Register<'info> {
    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(seeds = [SEED_AUTHORITY], bump)]
    pub authority: Account<'info, Authority>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Account<'info, Config>,

    #[account()]
    pub identity: Signer<'info>,

    #[account(address = config.mint)]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        seeds = [
            SEED_NODE,
            identity.key().as_ref()
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Node>(),
    )]
    pub node: Account<'info, Node>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub queue: Account<'info, Queue>,

    #[account(mut, seeds = [SEED_REGISTRY], bump)]
    pub registry: Account<'info, Registry>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,

    #[account(address = cronos_scheduler::ID)]
    pub scheduler_program: Program<'info, System>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub task: Account<'info, Task>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(
        init,
        payer = payer,
        associated_token::authority = node,
        associated_token::mint = mint,
    )]
    pub stake: Account<'info, TokenAccount>,
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, Register<'info>>) -> Result<()> {
    // Get accounts
    let _authority = &ctx.accounts.authority;
    let _config = &ctx.accounts.config;
    let identity = &mut ctx.accounts.identity;
    let node = &mut ctx.accounts.node;
    let _payer = &mut ctx.accounts.payer;
    let _queue = &ctx.accounts.queue;
    let registry = &mut ctx.accounts.registry;
    let _scheduler_program = &ctx.accounts.scheduler_program;
    let _system_program = &ctx.accounts.system_program;
    let stake = &mut ctx.accounts.stake;
    let _task = &mut ctx.accounts.task;

    // Get remaining accounts
    let _action = ctx.remaining_accounts.get(0).unwrap();
    // let queue = ctx.remaining_accounts.get(1).unwrap();
    // let task = ctx.remaining_accounts.get(2).unwrap();

    // Get bumps
    let _authority_bump = *ctx.bumps.get("authority").unwrap();

    // Add node to the registry
    registry.new_node(identity, node, stake)?;

    // TODO Add an action to the snapshot task to capture this node in the snapshot
    // TODO add a rotate_snapshot ix to the action

    

    Ok(())
}
