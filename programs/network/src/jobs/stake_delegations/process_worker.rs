use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
use anchor_spl::associated_token::get_associated_token_address;
use clockwork_utils::automation::AutomationResponse;

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
        Some(
            Instruction {
                program_id: crate::ID,
                accounts: crate::accounts::StakeDelegationsProcessDelegation {
                    config: config.key(),
                    delegation: delegation_pubkey,
                    delegation_stake: get_associated_token_address(
                        &delegation_pubkey,
                        &config.mint,
                    ),
                    registry: registry.key(),
                    automation: automation.key(),
                    token_program: anchor_spl::token::ID,
                    worker: worker.key(),
                    worker_stake: get_associated_token_address(&worker.key(), &config.mint),
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
        // This worker has no delegations. Move on to the next worker.
        Some(
            Instruction {
                program_id: crate::ID,
                accounts: crate::accounts::StakeDelegationsProcessWorker {
                    config: config.key(),
                    registry: registry.key(),
                    automation: automation.key(),
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

    Ok(AutomationResponse {
        dynamic_instruction,
        trigger: None,
    })
}
