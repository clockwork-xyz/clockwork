use anchor_lang::{prelude::*, InstructionData};
use clockwork_utils::automation::{Acc, AutomationResponse, Ix};

use crate::{instruction, state::*};

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
                    Acc::readonly(config.key(), false),
                    Acc::readonly(registry.key(), false),
                    Acc::readonly(automation.key(), true),
                    Acc::readonly(Worker::pubkey(0), false),
                ],
                data: instruction::StakeDelegationsProcessWorker.data(),
            })
        } else {
            None
        },
        trigger: None,
    })
}
