use {
    crate::{errors::*, state::*},
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
pub struct WorkerCreate<'info> {
    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(address = Config::pubkey())]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init,
        seeds = [
            SEED_FEE,
            worker.key().as_ref(),
        ],
        bump,
        payer = authority,
        space = 8 + size_of::<Fee>(),
    )]
    pub fee: Account<'info, Fee>,

    #[account(
        init,
        seeds = [
            SEED_PENALTY,
            worker.key().as_ref(),
        ],
        bump,
        payer = authority,
        space = 8 + size_of::<Penalty>(),
    )]
    pub penalty: Account<'info, Penalty>,

    #[account(address = config.mint)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut, 
        seeds = [SEED_REGISTRY],
        bump,
        constraint = !registry.locked @ ClockworkError::RegistryLocked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,

    #[account(constraint = signatory.key().ne(&authority.key()) @ ClockworkError::InvalidSignatory)]
    pub signatory: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(
        init,
        seeds = [
            SEED_WORKER,
            registry.total_workers.to_be_bytes().as_ref(),
        ],
        bump,
        payer = authority,
        space = 8 + size_of::<Worker>(),
    )]
    pub worker: Account<'info, Worker>,

    #[account(
        init,
        payer = authority,
        associated_token::authority = worker,
        associated_token::mint = mint,
    )]
    pub worker_tokens: Account<'info, TokenAccount>,

}

pub fn handler(ctx: Context<WorkerCreate>) -> Result<()> {
    // Get accounts
    let authority = &mut ctx.accounts.authority;
    let fee = &mut ctx.accounts.fee;
    let penalty = &mut ctx.accounts.penalty;
    let registry = &mut ctx.accounts.registry;
    let signatory = &mut ctx.accounts.signatory;
    let worker = &mut ctx.accounts.worker;

    // Initialize the worker accounts.
    worker.init(authority, registry.total_workers, signatory)?;
    fee.init(worker.key())?;
    penalty.init(worker.key())?;

    // Update the registry's worker counter.
    registry.total_workers = registry.total_workers.checked_add(1).unwrap();

    Ok(())
}
