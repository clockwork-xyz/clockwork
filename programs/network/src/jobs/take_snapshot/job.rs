use anchor_lang::{prelude::*, solana_program::system_program};
use clockwork_utils::automation::{
    anchor_sighash, AccountBuilder, Ix, AutomationResponse, PAYER_PUBKEY,
};

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
        dynamic_instruction: Some(Ix {
            program_id: crate::ID,
            accounts: vec![
                AccountBuilder::readonly(config.key(), false),
                AccountBuilder::mutable(PAYER_PUBKEY, true),
                AccountBuilder::readonly(registry.key(), false),
                AccountBuilder::mutable(
                    Snapshot::pubkey(registry.current_epoch.checked_add(1).unwrap()),
                    false,
                ),
                AccountBuilder::readonly(system_program::ID, false),
                AccountBuilder::readonly(automation.key(), true),
            ],
            data: anchor_sighash("take_snapshot_create_snapshot").to_vec(),
        }),
        trigger: None,
    })
}
