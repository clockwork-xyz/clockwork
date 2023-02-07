use anchor_lang::prelude::*;
use anchor_spl::associated_token::get_associated_token_address;
use clockwork_utils::automation::{
    anchor_sighash, AccountMetaData, InstructionData, AutomationResponse,
};

use crate::state::*;

#[derive(Accounts)]
pub struct UnstakePreprocess<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = Registry::pubkey(),
        constraint = registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = config.epoch_automation)]
    pub automation: Signer<'info>,

    #[account(address = unstake.pubkey())]
    pub unstake: Account<'info, Unstake>,
}

pub fn handler(ctx: Context<UnstakePreprocess>) -> Result<AutomationResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let automation = &ctx.accounts.automation;
    let unstake = &ctx.accounts.unstake;

    // Return next instruction for automation.
    Ok(AutomationResponse {
        next_instruction: Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(unstake.authority, false),
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new(unstake.delegation, false),
                AccountMetaData::new(registry.key(), false),
                AccountMetaData::new_readonly(automation.key(), true),
                AccountMetaData::new_readonly(anchor_spl::token::ID, false),
                AccountMetaData::new(unstake.key(), false),
                AccountMetaData::new_readonly(unstake.worker, false),
                AccountMetaData::new(
                    get_associated_token_address(&unstake.worker, &config.mint),
                    false,
                ),
            ],
            data: anchor_sighash("unstake_process").to_vec(),
        }),
        ..AutomationResponse::default()
    })
}
