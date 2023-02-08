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
        dynamic_instruction: Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::readonly(unstake.authority, false),
                AccountMetaData::readonly(config.key(), false),
                AccountMetaData::mutable(unstake.delegation, false),
                AccountMetaData::mutable(registry.key(), false),
                AccountMetaData::readonly(automation.key(), true),
                AccountMetaData::readonly(anchor_spl::token::ID, false),
                AccountMetaData::mutable(unstake.key(), false),
                AccountMetaData::readonly(unstake.worker, false),
                AccountMetaData::mutable(
                    get_associated_token_address(&unstake.worker, &config.mint),
                    false,
                ),
            ],
            data: anchor_sighash("unstake_process").to_vec(),
        }),
        ..AutomationResponse::default()
    })
}
