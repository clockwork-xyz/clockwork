use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
use anchor_spl::associated_token::get_associated_token_address;
use clockwork_utils::automation::AutomationResponse;

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
        dynamic_instruction: Some(
            Instruction {
                program_id: crate::ID,
                accounts: crate::accounts::UnstakeProcess {
                    authority: unstake.authority,
                    authority_tokens: config.key(),
                    config: unstake.delegation,
                    delegation: registry.key(),
                    registry: automation.key(),
                    automation: anchor_spl::token::ID,
                    token_program: unstake.key(),
                    unstake: unstake.worker,
                    worker: get_associated_token_address(&unstake.worker, &config.mint),
                    worker_tokens: get_associated_token_address(&unstake.worker, &config.mint),
                }
                .to_account_metas(Some(true)),
                data: crate::instruction::UnstakeProcess {}.data(),
            }
            .into(),
        ),
        ..AutomationResponse::default()
    })
}
