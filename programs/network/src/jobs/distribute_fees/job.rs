use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
use clockwork_utils::automation::AutomationResponse;

use crate::state::*;

#[derive(Accounts)]
pub struct DistributeFeesJob<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_REGISTRY],
        bump,
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = config.epoch_automation)]
    pub automation: Signer<'info>,
}

pub fn handler(ctx: Context<DistributeFeesJob>) -> Result<AutomationResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let registry = &mut ctx.accounts.registry;
    let automation = &ctx.accounts.automation;

    // Lock the registry.
    registry.locked = true;

    // Process the snapshot.
    Ok(AutomationResponse {
        dynamic_instruction: Some(
            Instruction {
                program_id: crate::ID,
                accounts: crate::accounts::DistributeFeesProcessSnapshot {
                    config: config.key(),
                    registry: registry.key(),
                    snapshot: Snapshot::pubkey(registry.current_epoch),
                    automation: automation.key(),
                }
                .to_account_metas(Some(true)),
                data: crate::instruction::DistributeFeesProcessSnapshot {}.data(),
            }
            .into(),
        ),
        trigger: None,
    })
}
