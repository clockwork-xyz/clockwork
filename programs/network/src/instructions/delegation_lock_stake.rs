use {
    crate::objects::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    anchor_spl::{
        associated_token::get_associated_token_address,
        token::{transfer, Mint, Token, TokenAccount, Transfer},
    },
    clockwork_utils::{anchor_sighash, AccountMetaData, CrankResponse, InstructionData},
};

#[derive(Accounts)]
pub struct DelegationLockStake<'info> {
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
        associated_token::authority = delegation,
        associated_token::mint = config.mint,
    )]
    pub delegation_stake: Account<'info, TokenAccount>,

    #[account(address = config.authorized_queue)]
    pub queue: Signer<'info>,

    #[account(
        address = Registry::pubkey(),
        constraint = registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account(
        mut,
        seeds = [
            SEED_WORKER,
            worker.id.to_be_bytes().as_ref()
        ],
        bump,
    )]
    pub worker: Account<'info, Worker>,

    #[account(
        associated_token::authority = worker,
        associated_token::mint = config.mint,
    )]
    pub worker_stake: Account<'info, TokenAccount>,
}

pub fn handler(ctx: Context<DelegationLockStake>) -> Result<CrankResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let delegation = &mut ctx.accounts.delegation;
    let delegation_stake = &mut ctx.accounts.delegation_stake;
    let queue = &ctx.accounts.queue;
    let registry = &ctx.accounts.registry;
    let token_program = &ctx.accounts.token_program;
    let worker = &ctx.accounts.worker;
    let worker_stake = &ctx.accounts.worker_stake;

    // Transfer tokens from delegation stake account to worker stake account.
    let amount = delegation_stake.amount;
    let bump = *ctx.bumps.get("delegation").unwrap();
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

    // Update the delegation's locked stake balance.
    delegation.locked_stake_amount = delegation.locked_stake_amount.checked_add(amount).unwrap();

    // Build next instruction for the queue.
    let next_instruction = if delegation
        .id
        .checked_add(1)
        .unwrap()
        .lt(&worker.total_delegations)
    {
        // This worker has more delegations, continue locking their stake.
        let next_delegation_pubkey =
            Delegation::pubkey(worker.key(), delegation.id.checked_add(1).unwrap());
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new(next_delegation_pubkey, false),
                AccountMetaData::new(
                    get_associated_token_address(&next_delegation_pubkey, &config.mint),
                    false,
                ),
                AccountMetaData::new_readonly(queue.key(), true),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(token_program.key(), false),
                AccountMetaData::new_readonly(worker.key(), false),
                AccountMetaData::new(worker_stake.key(), false),
            ],
            data: anchor_sighash("delegation_lock_stake").to_vec(),
        })
    } else if worker
        .id
        .checked_add(1)
        .unwrap()
        .lt(&registry.total_workers)
    {
        // TODO If this worker has no more delegations, move on to the next worker.
        // TODO What if the next worker has no delegations????
        None
    } else {
        // TODO If this is the last worker, move on to the next task.
        None
    };

    Ok(CrankResponse { next_instruction })
}
