use anchor_lang::{prelude::*, InstructionData};
use clockwork_utils::automation::{AutomationResponse, InstructionBuilder};

use crate::{network_program::instruction, state::*};

#[derive(Accounts)]
pub struct DeleteSnapshotJob<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = Registry::pubkey(),
        constraint = !registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = config.epoch_automation)]
    pub automation: Signer<'info>,
}

pub fn handler(ctx: Context<DeleteSnapshotJob>) -> Result<AutomationResponse> {
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let automation = &mut ctx.accounts.automation;

    Ok(AutomationResponse {
        dynamic_instruction: Some(
            InstructionBuilder::new(crate::ID)
                .readonly_account(config.key())
                .readonly_account(registry.key())
                .readonly_account(Snapshot::pubkey(
                    registry.current_epoch.checked_sub(1).unwrap(),
                ))
                .signer(automation.key())
                .data(instruction::DeleteSnapshotProcessSnapshot {}.data())
                .build(),
        ),
        trigger: None,
    })
}
