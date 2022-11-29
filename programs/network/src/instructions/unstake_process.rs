use clockwork_utils::{anchor_sighash, AccountMetaData, InstructionData};

use {
    crate::{errors::*, state::*},
    anchor_lang::prelude::*,
    anchor_spl::token::{transfer, Token, TokenAccount, Transfer},
    clockwork_utils::ThreadResponse,
};

#[derive(Accounts)]
pub struct UnstakeProcess<'info> {
    #[account()]
    pub authority: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::authority = delegation.authority,
        associated_token::mint = config.mint,
    )]
    pub authority_tokens: Box<Account<'info, TokenAccount>>,

    #[account(address = Config::pubkey())]
    pub config: Box<Account<'info, Config>>,

    #[account(
        mut,
        seeds = [
            SEED_DELEGATION,
            delegation.worker.as_ref(),
            delegation.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = authority,
        has_one = worker,
    )]
    pub delegation: Box<Account<'info, Delegation>>,

    #[account(
        mut,
        seeds = [SEED_REGISTRY],
        bump,
    )]
    pub registry: Box<Account<'info, Registry>>,

    #[account(address = config.epoch_thread)]
    pub thread: Signer<'info>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(
        mut,
        seeds = [
            SEED_UNSTAKE,
            unstake.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = authority,
        has_one = delegation
    )]
    pub unstake: Box<Account<'info, Unstake>>,

    #[account(address = worker.pubkey())]
    pub worker: Account<'info, Worker>,

    #[account(
        mut,
        associated_token::authority = worker,
        associated_token::mint = config.mint,
    )]
    pub worker_tokens: Box<Account<'info, TokenAccount>>,
}

pub fn handler(ctx: Context<UnstakeProcess>) -> Result<ThreadResponse> {
    // Get accounts.
    let authority = &ctx.accounts.authority;
    let authority_tokens = &ctx.accounts.authority_tokens;
    let config = &ctx.accounts.config;
    let delegation = &mut ctx.accounts.delegation;
    let registry = &mut ctx.accounts.registry;
    let thread = &ctx.accounts.thread;
    let token_program = &ctx.accounts.token_program;
    let unstake = &ctx.accounts.unstake;
    let worker = &ctx.accounts.worker;
    let worker_tokens = &ctx.accounts.worker_tokens;

    // Verify the unstake amount is valid.
    require!(
        unstake.amount.le(&delegation.stake_amount),
        ClockworkError::InvalidUnstakeAmount
    );

    // Transfer tokens from the worker to the authority.
    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: worker_tokens.to_account_info(),
                to: authority_tokens.to_account_info(),
                authority: worker.to_account_info(),
            },
            &[&[SEED_WORKER, worker.id.to_be_bytes().as_ref()]],
        ),
        unstake.amount,
    )?;

    // Decrement the delegations locked stake balacne by the requested unstake amount.
    delegation.stake_amount = delegation.stake_amount.checked_sub(unstake.amount).unwrap();

    // Close the unstake account by transfering all lamports to the authority.
    let balance = unstake.to_account_info().lamports();
    **unstake.to_account_info().try_borrow_mut_lamports()? = unstake
        .to_account_info()
        .lamports()
        .checked_sub(balance)
        .unwrap();
    **authority.to_account_info().try_borrow_mut_lamports()? = authority
        .to_account_info()
        .lamports()
        .checked_add(balance)
        .unwrap();

    // If this is the last unstake, then reset the registry's unstake counter.
    if unstake
        .id
        .checked_add(1)
        .unwrap()
        .eq(&registry.total_unstakes)
    {
        registry.total_unstakes = 0;
    }

    // Build next instruction for the thread.
    let next_instruction = if unstake
        .id
        .checked_add(1)
        .unwrap()
        .lt(&registry.total_unstakes)
    {
        let next_unstake_pubkey = Unstake::pubkey(unstake.id.checked_add(1).unwrap());
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(thread.key(), true),
                AccountMetaData::new_readonly(next_unstake_pubkey, false),
            ],
            data: anchor_sighash("unstake_preprocess").to_vec(),
        })
    } else {
        // This is the last unstake. Reset the registry's unstake counter.
        registry.total_unstakes = 0;

        // Move on to staking delegations.
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(thread.key(), true),
                AccountMetaData::new_readonly(Worker::pubkey(0), false),
            ],
            data: anchor_sighash("worker_delegations_stake").to_vec(),
        })
    };

    Ok(ThreadResponse {
        next_instruction,
        ..ThreadResponse::default()
    })
}
