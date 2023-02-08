use anchor_lang::prelude::*;
use clockwork_utils::automation::{
    anchor_sighash, AccountBuilder, Ix, AutomationResponse,
};

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
        dynamic_instruction: Some(Ix {
            program_id: crate::ID,
            accounts: vec![
                AccountBuilder::readonly(config.key(), false),
                AccountBuilder::readonly(registry.key(), false),
                AccountBuilder::readonly(Snapshot::pubkey(registry.current_epoch), false),
                AccountBuilder::readonly(automation.key(), true),
            ],
            data: anchor_sighash("distribute_fees_process_snapshot").to_vec(),
        }),
        trigger: None,
    })
}
