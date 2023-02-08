use {
    crate::state::*,
    anchor_lang::prelude::*,
    clockwork_utils::automation::{
        anchor_sighash, AccountMetaData, InstructionData, AutomationResponse,
    },
};

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
        dynamic_instruction: Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::readonly(config.key(), false),
                AccountMetaData::readonly(registry.key(), false),
                AccountMetaData::mutable(
                    Snapshot::pubkey(registry.current_epoch.checked_sub(1).unwrap()),
                    false,
                ),
                AccountMetaData::mutable(automation.key(), true),
            ],
            data: anchor_sighash("delete_snapshot_process_snapshot").to_vec(),
        }),
        trigger: None,
    })
}
