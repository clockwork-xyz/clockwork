use {
    crate::objects::*,
    anchor_lang::prelude::*,
    anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct NodeStake<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(address = node.pubkey())]
    pub node: Account<'info, Node>,

    #[account(
        mut,
        associated_token::authority = node,
        associated_token::mint = mint,
    )]
    pub node_stake: Account<'info, TokenAccount>,

    #[account(address = config.mint)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(
        mut,
        associated_token::authority = signer,
        associated_token::mint = mint,
    )]
    pub tokens: Account<'info, TokenAccount>,
}

pub fn handler(ctx: Context<NodeStake>, amount: u64) -> Result<()> {
    let node_stake = &mut ctx.accounts.node_stake;
    let signer = &mut ctx.accounts.signer;
    let token_program = &ctx.accounts.token_program;
    let tokens = &mut ctx.accounts.tokens;

    transfer(
        CpiContext::new(
            token_program.to_account_info(),
            Transfer {
                from: tokens.to_account_info(),
                to: node_stake.to_account_info(),
                authority: signer.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}
