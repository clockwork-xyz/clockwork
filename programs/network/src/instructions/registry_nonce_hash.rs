use clockwork_utils::CrankResponse;

use {crate::objects::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct RegistryNonceHash<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(address = config.hasher_queue)]
    pub queue: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_REGISTRY],
        bump
    )]
    pub registry: Account<'info, Registry>,
}

pub fn handler(ctx: Context<RegistryNonceHash>) -> Result<CrankResponse> {
    let registry = &mut ctx.accounts.registry;
    registry.hash_nonce()?;
    Ok(CrankResponse::default())
}
