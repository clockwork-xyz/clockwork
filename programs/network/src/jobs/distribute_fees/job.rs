use anchor_lang::{prelude::*, InstructionData};
use clockwork_utils::automation::{AutomationResponse, InstructionBuilder};

use crate::{instruction, state::*};

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
            InstructionBuilder::new(crate::ID)
                .readonly_account(config.key())
                .readonly_account(registry.key())
                .readonly_account(Snapshot::pubkey(registry.current_epoch))
                .signer(automation.key())
                .data(instruction::DistributeFeesProcessSnapshot {}.data())
                .build(),
        ),
        trigger: None,
    })
}
