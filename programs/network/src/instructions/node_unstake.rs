use {
    crate::objects::*,
    anchor_lang::prelude::*,
    anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct NodeUnstake<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = node.pubkey(),
        has_one = authority,
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
        associated_token::authority = authority,
        associated_token::mint = mint,
    )]
    pub tokens: Account<'info, TokenAccount>,
}

pub fn handler(ctx: Context<NodeUnstake>, amount: u64) -> Result<()> {
    // Get accounts
    let node = &ctx.accounts.node;
    let node_stake = &mut ctx.accounts.node_stake;
    let token_program = &ctx.accounts.token_program;
    let tokens = &mut ctx.accounts.tokens;

    // Transfer trokens from stake account to authority's stake account
    let bump = *ctx.bumps.get("node").unwrap();
    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: node_stake.to_account_info(),
                to: tokens.to_account_info(),
                authority: node.to_account_info(),
            },
            &[&[SEED_NODE, node.id.to_be_bytes().as_ref(), &[bump]]],
        ),
        amount,
    )?;

    Ok(())
}
