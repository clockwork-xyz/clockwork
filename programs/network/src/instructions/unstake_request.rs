use {
    crate::objects::*,
    anchor_lang::prelude::*,
    anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct DelegationRequestUnstake<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

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

    #[account(
        seeds = [
            SEED_WORKER,
            worker.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = authority,
    )]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<DelegationRequestUnstake>, amount: u64) -> Result<()> {
    // Get accounts
    let worker = &ctx.accounts.worker;
    let token_program = &ctx.accounts.token_program;
    let tokens = &mut ctx.accounts.tokens;

    // Transfer trokens from stake account to authority's stake account
    // let bump = *ctx.bumps.get("worker").unwrap();
    // transfer(
    //     CpiContext::new_with_signer(
    //         token_program.to_account_info(),
    //         Transfer {
    //             from: worker_stake.to_account_info(),
    //             to: tokens.to_account_info(),
    //             authority: worker.to_account_info(),
    //         },
    //         &[&[SEED_NODE, worker.id.to_be_bytes().as_ref(), &[bump]]],
    //     ),
    //     amount,
    // )?;

    Ok(())
}
