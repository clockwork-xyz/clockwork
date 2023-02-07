use anchor_lang::prelude::*;
use clockwork_utils::automation::{
    anchor_sighash, AccountMetaData, InstructionData, AutomationResponse,
};

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
        next_instruction: if registry.total_unstakes.gt(&0) {
            Some(InstructionData {
                program_id: crate::ID,
                accounts: vec![
                    AccountMetaData::new_readonly(config.key(), false),
                    AccountMetaData::new_readonly(registry.key(), false),
                    AccountMetaData::new_readonly(automation.key(), true),
                    AccountMetaData::new_readonly(Unstake::pubkey(0), false),
                ],
                data: anchor_sighash("unstake_preprocess").to_vec(),
            })
        } else {
            None
        },
        trigger: None,
    })
}
