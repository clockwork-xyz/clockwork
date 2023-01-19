use {
    crate::state::*,
    anchor_lang::prelude::*,
    anchor_spl::token::{transfer, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct DelegationWithdraw<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        associated_token::authority = authority,
        associated_token::mint = config.mint,
    )]
    pub authority_tokens: Account<'info, TokenAccount>,

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
        has_one = authority,
    )]
    pub delegation: Account<'info, Delegation>,

    #[account(
        mut,
        associated_token::authority = delegation,
        associated_token::mint = config.mint,
    )]
    pub delegation_tokens: Account<'info, TokenAccount>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,
}

pub fn handler(ctx: Context<DelegationWithdraw>, amount: u64) -> Result<()> {
    // Get accounts.
    let authority_tokens = &ctx.accounts.authority_tokens;
    let delegation = &ctx.accounts.delegation;
    let delegation_tokens = &ctx.accounts.delegation_tokens;
    let token_program = &ctx.accounts.token_program;

    // Transfer tokens from authority tokens to delegation
    let bump = *ctx.bumps.get("delegation").unwrap();
    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: delegation_tokens.to_account_info(),
                to: authority_tokens.to_account_info(),
                authority: delegation.to_account_info(),
            },
            &[&[
                SEED_DELEGATION,
                delegation.worker.as_ref(),
                delegation.id.to_be_bytes().as_ref(),
                &[bump],
            ]],
        ),
        amount,
    )?;

    Ok(())
}
