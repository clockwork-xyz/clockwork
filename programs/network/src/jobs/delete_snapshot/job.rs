use anchor_lang::prelude::*;
use clockwork_utils::automation::AutomationResponse;

use crate::{
    delete_snapshot::process_snapshot::DeleteSnapshotProcessSnapshotInstruction, state::*,
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

    let ix = DeleteSnapshotProcessSnapshotInstruction::build(
        config.key(),
        registry.key(),
        Snapshot::pubkey(registry.current_epoch.checked_sub(1).unwrap()),
        automation.key(),
    );

    Ok(AutomationResponse {
        dynamic_instruction: Some(ix),
        trigger: None,
    })
}
