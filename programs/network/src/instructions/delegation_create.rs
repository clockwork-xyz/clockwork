use {
    crate::objects::*,
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct DelegationCreate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        associated_token::authority = authority,
        associated_token::mint = mint,
    )]
    pub authority_tokens: Account<'info, TokenAccount>,

    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(address = config.mint)]
    pub mint: Account<'info, Mint>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(
        seeds = [
            SEED_WORKER,
            worker.id.to_be_bytes().as_ref(),
        ],
        bump
    )]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<DelegationCreate>) -> Result<()> {
    // let authority = &mut ctx.accounts.authority;
    // let authority_tokens = &mut ctx.accounts.authority_tokens;
    // let token_program = &ctx.accounts.token_program;

    // transfer(
    //     CpiContext::new(
    //         token_program.to_account_info(),
    //         Transfer {
    //             from: tokens.to_account_info(),
    //             to: worker_stake.to_account_info(),
    //             authority: signer.to_account_info(),
    //         },
    //     ),
    //     amount,
    // )?;

    Ok(())
}
