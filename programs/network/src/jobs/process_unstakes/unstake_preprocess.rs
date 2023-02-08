use anchor_lang::{prelude::*, InstructionData};
use anchor_spl::associated_token::get_associated_token_address;
use clockwork_utils::automation::{Acc, AutomationResponse, Ix};

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
        dynamic_instruction: Some(Ix {
            program_id: crate::ID,
            accounts: vec![
                Acc::readonly(unstake.authority, false),
                Acc::readonly(config.key(), false),
                Acc::mutable(unstake.delegation, false),
                Acc::mutable(registry.key(), false),
                Acc::readonly(automation.key(), true),
                Acc::readonly(anchor_spl::token::ID, false),
                Acc::mutable(unstake.key(), false),
                Acc::readonly(unstake.worker, false),
                Acc::mutable(
                    get_associated_token_address(&unstake.worker, &config.mint),
                    false,
                ),
            ],
            data: instruction::UnstakeProcess {}.data(),
        }),
        ..AutomationResponse::default()
    })
}
