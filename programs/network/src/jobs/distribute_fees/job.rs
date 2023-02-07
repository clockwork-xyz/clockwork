use anchor_lang::prelude::*;
use clockwork_utils::automation::{
    anchor_sighash, AccountMetaData, AutomationResponse, InstructionData,
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
        next_instruction: Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(Snapshot::pubkey(registry.current_epoch), false),
                AccountMetaData::new_readonly(automation.key(), true),
            ],
            data: anchor_sighash("distribute_fees_process_snapshot").to_vec(),
        }),
        trigger: None,
    })
}
