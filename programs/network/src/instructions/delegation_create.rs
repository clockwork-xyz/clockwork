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
pub struct DelegationCreate<'info> {
    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        init,
        seeds = [
            SEED_DELEGATION,
            worker.key().as_ref(),
            worker.total_delegations.to_be_bytes().as_ref(),
        ],
        bump,
        payer = authority,
        space = 8 + size_of::<Delegation>(),
    )]
    pub delegation: Account<'info, Delegation>,

    #[account(
        init,
        payer = authority,
        associated_token::authority = delegation,
        associated_token::mint = mint,
    )]
    pub delegation_tokens: Account<'info, TokenAccount>,

    #[account(address = config.mint)]
    pub mint: Account<'info, Mint>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(
        mut,
        seeds = [
            SEED_WORKER,
            worker.id.to_be_bytes().as_ref(),
        ],
        bump
    )]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<DelegationCreate>) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let delegation = &mut ctx.accounts.delegation;
    let worker = &mut ctx.accounts.worker;

    // Initialize the delegation account.
    delegation.init(authority.key(), worker.total_delegations, worker.key())?;

    // Increment the worker's total delegations counter.
    worker.total_delegations = worker.total_delegations.checked_add(1).unwrap();

    Ok(())
}
