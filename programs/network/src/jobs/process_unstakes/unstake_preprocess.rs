use anchor_lang::{prelude::*, InstructionData};
use anchor_spl::associated_token::get_associated_token_address;
use clockwork_utils::automation::{
    AutomationResponse, SerializableAccount, SerializableInstruction,
};

use crate::{instruction, state::*};

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
        dynamic_instruction: Some(SerializableInstruction {
            program_id: crate::ID,
            accounts: vec![
                SerializableAccount::readonly(unstake.authority, false),
                SerializableAccount::readonly(config.key(), false),
                SerializableAccount::mutable(unstake.delegation, false),
                SerializableAccount::mutable(registry.key(), false),
                SerializableAccount::readonly(automation.key(), true),
                SerializableAccount::readonly(anchor_spl::token::ID, false),
                SerializableAccount::mutable(unstake.key(), false),
                SerializableAccount::readonly(unstake.worker, false),
                SerializableAccount::mutable(
                    get_associated_token_address(&unstake.worker, &config.mint),
                    false,
                ),
            ],
            data: instruction::UnstakeProcess {}.data(),
        }),
        ..AutomationResponse::default()
    })
}
