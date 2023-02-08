use anchor_lang::prelude::*;
use anchor_spl::associated_token::get_associated_token_address;
use clockwork_utils::automation::{
    anchor_sighash, AccountBuilder, Ix, AutomationResponse,
};

use crate::state::*;

#[derive(Accounts)]
pub struct StakeDelegationsProcessWorker<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = Registry::pubkey(),
        constraint = registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = config.epoch_automation)]
    pub automation: Signer<'info>,

    #[account(address = worker.pubkey())]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<StakeDelegationsProcessWorker>) -> Result<AutomationResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let automation = &ctx.accounts.automation;
    let worker = &ctx.accounts.worker;

    // Build the next instruction for the automation.
    let dynamic_instruction = if worker.total_delegations.gt(&0) {
        // This worker has delegations. Stake their deposits.
        let delegation_pubkey = Delegation::pubkey(worker.key(), 0);
        Some(Ix {
            program_id: crate::ID,
            accounts: vec![
                AccountBuilder::readonly(config.key(), false),
                AccountBuilder::mutable(delegation_pubkey, false),
                AccountBuilder::mutable(
                    get_associated_token_address(&delegation_pubkey, &config.mint),
                    false,
                ),
                AccountBuilder::readonly(registry.key(), false),
                AccountBuilder::readonly(automation.key(), true),
                AccountBuilder::readonly(anchor_spl::token::ID, false),
                AccountBuilder::readonly(worker.key(), false),
                AccountBuilder::mutable(
                    get_associated_token_address(&worker.key(), &config.mint),
                    false,
                ),
            ],
            data: anchor_sighash("stake_delegations_process_delegation").to_vec(),
        })
    } else if worker
        .id
        .checked_add(1)
        .unwrap()
        .lt(&registry.total_workers)
    {
        // This worker has no delegations. Move on to the next worker.
        Some(Ix {
            program_id: crate::ID,
            accounts: vec![
                AccountBuilder::readonly(config.key(), false),
                AccountBuilder::readonly(registry.key(), false),
                AccountBuilder::readonly(automation.key(), true),
                AccountBuilder::readonly(
                    Worker::pubkey(worker.id.checked_add(1).unwrap()),
                    false,
                ),
            ],
            data: anchor_sighash("stake_delegations_process_worker").to_vec(),
        })
    } else {
        None
    };

    Ok(AutomationResponse {
        dynamic_instruction,
        trigger: None,
    })
}
