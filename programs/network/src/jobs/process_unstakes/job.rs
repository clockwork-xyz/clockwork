use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
use clockwork_utils::automation::AutomationResponse;

use crate::state::*;

#[derive(Accounts)]
pub struct ProcessUnstakesJob<'info> {
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

pub fn handler(ctx: Context<ProcessUnstakesJob>) -> Result<AutomationResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let automation = &ctx.accounts.automation;

    // Return next instruction for automation.
    Ok(AutomationResponse {
        dynamic_instruction: if registry.total_unstakes.gt(&0) {
            Some(
                Instruction {
                    program_id: crate::ID,
                    accounts: crate::accounts::UnstakePreprocess {
                        config: config.key(),
                        registry: registry.key(),
                        automation: automation.key(),
                        unstake: Unstake::pubkey(0),
                    }
                    .to_account_metas(Some(true)),
                    data: crate::instruction::UnstakePreprocess {}.data(),
                }
                .into(),
            )
        } else {
            None
        },
        trigger: None,
    })
}
