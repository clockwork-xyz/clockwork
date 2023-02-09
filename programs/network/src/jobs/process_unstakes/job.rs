use anchor_lang::{prelude::*, InstructionData};
use clockwork_utils::automation::{
    AutomationResponse, SerializableAccount, SerializableInstruction,
};

use crate::{instruction, state::*};

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
            Some(SerializableInstruction {
                program_id: crate::ID,
                accounts: vec![
                    SerializableAccount::readonly(config.key(), false),
                    SerializableAccount::readonly(registry.key(), false),
                    SerializableAccount::readonly(automation.key(), true),
                    SerializableAccount::readonly(Unstake::pubkey(0), false),
                ],
                data: instruction::UnstakePreprocess {}.data(),
            })
        } else {
            None
        },
        trigger: None,
    })
}
