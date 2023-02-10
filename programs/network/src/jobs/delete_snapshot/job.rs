use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
use clockwork_utils::automation::AutomationResponse;

use crate::state::*;

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
            Instruction {
                program_id: crate::ID,
                accounts: crate::accounts::DeleteSnapshotProcessSnapshot {
                    config: config.key(),
                    registry: registry.key(),
                    snapshot: Snapshot::pubkey(registry.current_epoch.checked_sub(1).unwrap()),
                    automation: automation.key(),
                }
                .to_account_metas(Some(true)),
                data: crate::instruction::DeleteSnapshotProcessSnapshot {}.data(),
            }
            .into(),
        ),
        trigger: None,
    })
}
