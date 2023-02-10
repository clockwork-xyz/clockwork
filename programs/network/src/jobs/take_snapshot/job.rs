use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, system_program},
    InstructionData,
};
use clockwork_utils::automation::{AutomationResponse, PAYER_PUBKEY};

use crate::state::*;

#[derive(Accounts)]
pub struct TakeSnapshotJob<'info> {
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

pub fn handler(ctx: Context<TakeSnapshotJob>) -> Result<AutomationResponse> {
    // Get accounts
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let automation = &ctx.accounts.automation;

    Ok(AutomationResponse {
        dynamic_instruction: Some(
            Instruction {
                program_id: crate::ID,
                accounts: crate::accounts::TakeSnapshotCreateSnapshot {
                    config: config.key(),
                    payer: PAYER_PUBKEY,
                    registry: registry.key(),
                    snapshot: Snapshot::pubkey(registry.current_epoch.checked_add(1).unwrap()),
                    system_program: system_program::ID,
                    automation: automation.key(),
                }
                .to_account_metas(Some(true)),
                data: crate::instruction::TakeSnapshotCreateSnapshot {}.data(),
            }
            .into(),
        ),
        trigger: None,
    })
}
