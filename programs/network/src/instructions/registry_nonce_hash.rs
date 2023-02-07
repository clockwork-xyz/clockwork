use clockwork_utils::automation::AutomationResponse;

use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct RegistryNonceHash<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_REGISTRY],
        bump
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = config.hasher_automation)]
    pub automation: Signer<'info>,
}

pub fn handler(ctx: Context<RegistryNonceHash>) -> Result<AutomationResponse> {
    let registry = &mut ctx.accounts.registry;
    registry.hash_nonce()?;
    Ok(AutomationResponse::default())
}
