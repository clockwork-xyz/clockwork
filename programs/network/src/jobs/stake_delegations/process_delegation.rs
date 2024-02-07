use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
use anchor_spl::{
    associated_token::get_associated_token_address,
    token::{transfer, Token, TokenAccount, Transfer},
};
use clockwork_utils::thread::ThreadResponse;

use crate::state::*;

#[derive(Accounts)]
pub struct StakeDelegationsProcessDelegation<'info> {
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
        has_one = worker
    )]
    pub delegation: Account<'info, Delegation>,

    #[account(
        mut,
        associated_token::authority = delegation,
        associated_token::mint = config.mint,
    )]
    pub delegation_stake: Account<'info, TokenAccount>,

    #[account(
        address = Registry::pubkey(),
        constraint = registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = config.epoch_thread)]
    pub thread: Signer<'info>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(address = worker.pubkey())]
    pub worker: Account<'info, Worker>,

    #[account(
        mut,
        associated_token::authority = worker,
        associated_token::mint = config.mint,
    )]
    pub worker_stake: Account<'info, TokenAccount>,
}

pub fn handler(ctx: Context<StakeDelegationsProcessDelegation>) -> Result<ThreadResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let delegation = &mut ctx.accounts.delegation;
    let delegation_stake = &mut ctx.accounts.delegation_stake;
    let registry = &ctx.accounts.registry;
    let thread = &ctx.accounts.thread;
    let token_program = &ctx.accounts.token_program;
    let worker = &ctx.accounts.worker;
    let worker_stake = &ctx.accounts.worker_stake;

    // Transfer tokens from delegation to worker account.
    let amount = delegation_stake.amount;
    let bump = ctx.bumps.delegation;
    transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            Transfer {
                from: delegation_stake.to_account_info(),
                to: worker_stake.to_account_info(),
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

    // Update the delegation's stake amount.
    delegation.stake_amount = delegation.stake_amount.checked_add(amount).unwrap();

    // Build next instruction for the thread.
    let dynamic_instruction = if delegation
        .id
        .checked_add(1)
        .unwrap()
        .lt(&worker.total_delegations)
    {
        // This worker has more delegations, continue locking their stake.
        let next_delegation_pubkey =
            Delegation::pubkey(worker.key(), delegation.id.checked_add(1).unwrap());
        Some(
            Instruction {
                program_id: crate::ID,
                accounts: crate::accounts::StakeDelegationsProcessDelegation {
                    config: config.key(),
                    delegation: next_delegation_pubkey,
                    delegation_stake: get_associated_token_address(
                        &next_delegation_pubkey,
                        &config.mint,
                    ),
                    registry: registry.key(),
                    thread: thread.key(),
                    token_program: token_program.key(),
                    worker: worker.key(),
                    worker_stake: worker_stake.key(),
                }
                .to_account_metas(Some(true)),
                data: crate::instruction::StakeDelegationsProcessDelegation {}.data(),
            }
            .into(),
        )
    } else if worker
        .id
        .checked_add(1)
        .unwrap()
        .lt(&registry.total_workers)
    {
        // This worker has no more delegations, move on to the next worker.
        Some(
            Instruction {
                program_id: crate::ID,
                accounts: crate::accounts::StakeDelegationsProcessWorker {
                    config: config.key(),
                    registry: registry.key(),
                    thread: thread.key(),
                    worker: Worker::pubkey(worker.id.checked_add(1).unwrap()),
                }
                .to_account_metas(Some(true)),
                data: crate::instruction::StakeDelegationsProcessWorker {}.data(),
            }
            .into(),
        )
    } else {
        None
    };

    Ok(ThreadResponse {
        dynamic_instruction,
        close_to: None,
        trigger: None,
    })
}
