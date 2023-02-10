use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
use clockwork_utils::automation::AutomationResponse;

use crate::state::*;

#[derive(Accounts)]
pub struct StakeDelegationsJob<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = Registry::pubkey(),
        constraint = registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = config.epoch_automation)]
    pub automation: Signer<'info>,
}

pub fn handler(ctx: Context<StakeDelegationsJob>) -> Result<AutomationResponse> {
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let automation = &ctx.accounts.automation;

    Ok(AutomationResponse {
        dynamic_instruction: if registry.total_workers.gt(&0) {
            Some(
                Instruction {
                    program_id: crate::ID,
                    accounts: crate::accounts::StakeDelegationsProcessWorker {
                        config: config.key(),
                        registry: registry.key(),
                        automation: automation.key(),
                        worker: Worker::pubkey(0),
                    }
                    .to_account_metas(Some(true)),
                    data: crate::instruction::StakeDelegationsProcessWorker {}.data(),
                }
                .into(),
            )
        } else {
            None
        },
        trigger: None,
    })
}
