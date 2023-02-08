use anchor_lang::{prelude::*, InstructionData};
use anchor_spl::associated_token::get_associated_token_address;
use clockwork_utils::automation::{Acc, AutomationResponse, Ix};

use crate::{instruction, state::*};

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
                Acc::readonly(config.key(), false),
                Acc::mutable(delegation_pubkey, false),
                Acc::mutable(
                    get_associated_token_address(&delegation_pubkey, &config.mint),
                    false,
                ),
                Acc::readonly(registry.key(), false),
                Acc::readonly(automation.key(), true),
                Acc::readonly(anchor_spl::token::ID, false),
                Acc::readonly(worker.key(), false),
                Acc::mutable(
                    get_associated_token_address(&worker.key(), &config.mint),
                    false,
                ),
            ],
            data: instruction::StakeDelegationsProcessDelegation {}.data(),
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
                Acc::readonly(config.key(), false),
                Acc::readonly(registry.key(), false),
                Acc::readonly(automation.key(), true),
                Acc::readonly(Worker::pubkey(worker.id.checked_add(1).unwrap()), false),
            ],
            data: instruction::StakeDelegationsProcessWorker {}.data(),
        })
    } else {
        None
    };

    Ok(AutomationResponse {
        dynamic_instruction,
        trigger: None,
    })
}
