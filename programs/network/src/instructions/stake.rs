use {
    crate::state::*,
    anchor_lang::prelude::*,
    anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Stake<'info> {
    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub identity: Signer<'info>,

    #[account(
        seeds = [
            SEED_NODE,
            identity.key().as_ref()
        ],
        bump = node.bump,
    )]
    pub node: Account<'info, Node>,

    #[account(
        mut,
        associated_token::authority = node,
        associated_token::mint = mint,
    )]
    pub node_stake: Account<'info, TokenAccount>,

    #[account(address = config.mint)]
    pub mint: Account<'info, Mint>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(
        mut,
        associated_token::authority = identity,
        associated_token::mint = mint,
    )]
    pub tokens: Account<'info, TokenAccount>,
}

pub fn handler(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let identity = &mut ctx.accounts.identity;
    let node_stake = &mut ctx.accounts.node_stake;
    let token_program = &ctx.accounts.token_program;
    let tokens = &mut ctx.accounts.tokens;

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: tokens.to_account_info(),
                to: node_stake.to_account_info(),
                authority: identity.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}
