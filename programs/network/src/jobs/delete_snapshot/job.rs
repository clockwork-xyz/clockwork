use {
    crate::state::*,
    anchor_lang::prelude::*,
    clockwork_utils::automation::{
        anchor_sighash, AccountBuilder, Ix, AutomationResponse,
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
        dynamic_instruction: Some(Ix {
            program_id: crate::ID,
            accounts: vec![
                AccountBuilder::readonly(config.key(), false),
                AccountBuilder::readonly(registry.key(), false),
                AccountBuilder::mutable(
                    Snapshot::pubkey(registry.current_epoch.checked_sub(1).unwrap()),
                    false,
                ),
                AccountBuilder::mutable(automation.key(), true),
            ],
            data: anchor_sighash("delete_snapshot_process_snapshot").to_vec(),
        }),
        trigger: None,
    })
}
