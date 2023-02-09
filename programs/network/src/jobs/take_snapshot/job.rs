use anchor_lang::{prelude::*, solana_program::system_program, InstructionData};
use clockwork_utils::automation::{
    AutomationResponse, SerializableAccount, SerializableInstruction, PAYER_PUBKEY,
};

use crate::{instruction, state::*};

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
        dynamic_instruction: Some(SerializableInstruction {
            program_id: crate::ID,
            accounts: vec![
                SerializableAccount::readonly(config.key(), false),
                SerializableAccount::mutable(PAYER_PUBKEY, true),
                SerializableAccount::readonly(registry.key(), false),
                SerializableAccount::mutable(
                    Snapshot::pubkey(registry.current_epoch.checked_add(1).unwrap()),
                    false,
                ),
                SerializableAccount::readonly(system_program::ID, false),
                SerializableAccount::readonly(automation.key(), true),
            ],
            data: instruction::TakeSnapshotCreateSnapshot {}.data(),
        }),
        trigger: None,
    })
}
