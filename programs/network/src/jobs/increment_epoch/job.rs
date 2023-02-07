use anchor_lang::prelude::*;
use clockwork_utils::automation::AutomationResponse;

use crate::state::*;

#[derive(Accounts)]
pub struct EpochCutover<'info> {
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

pub fn handler(ctx: Context<EpochCutover>) -> Result<AutomationResponse> {
    let registry = &mut ctx.accounts.registry;
    registry.current_epoch = registry.current_epoch.checked_add(1).unwrap();
    registry.locked = false;
    Ok(AutomationResponse {
        next_instruction: None,
        trigger: None,
    })
}
