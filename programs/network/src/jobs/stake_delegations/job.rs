use anchor_lang::prelude::*;
use clockwork_utils::automation::{
    anchor_sighash, AccountBuilder, Ix, AutomationResponse,
};

use crate::state::*;

#[derive(Accounts)]
pub struct StakeDelegationsJob<'info> {
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

pub fn handler(ctx: Context<StakeDelegationsJob>) -> Result<AutomationResponse> {
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let automation = &ctx.accounts.automation;

    Ok(AutomationResponse {
        dynamic_instruction: if registry.total_workers.gt(&0) {
            Some(Ix {
                program_id: crate::ID,
                accounts: vec![
                    AccountBuilder::readonly(config.key(), false),
                    AccountBuilder::readonly(registry.key(), false),
                    AccountBuilder::readonly(automation.key(), true),
                    AccountBuilder::readonly(Worker::pubkey(0), false),
                ],
                data: anchor_sighash("stake_delegations_process_worker").to_vec(),
            })
        } else {
            None
        },
        trigger: None,
    })
}
