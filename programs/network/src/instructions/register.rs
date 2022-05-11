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
#[instruction(node_bump: u8)]
pub struct Register<'info> {
    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
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
        payer = identity,
        space = 8 + size_of::<Node>(),
    )]
    pub node: Account<'info, Node>,

    #[account(
        mut,
        seeds = [SEED_REGISTRY],
        bump = registry.bump,
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(
        init,
        payer = identity,
        associated_token::authority = node,
        associated_token::mint = mint,
    )]
    pub tokens: Account<'info, TokenAccount>,
}

pub fn handler(ctx: Context<Register>, node_bump: u8) -> Result<()> {
    let identity = &mut ctx.accounts.identity;
    let node = &mut ctx.accounts.node;
    let registry = &mut ctx.accounts.registry;
    let tokens = &mut ctx.accounts.tokens;

    registry.new_node(identity, node, node_bump, tokens)?;

    // TODO add an action to the task

    Ok(())
}
