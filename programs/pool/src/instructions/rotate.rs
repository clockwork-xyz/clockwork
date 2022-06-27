use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[account(delegate: Pubkey)]
pub struct Rotate<'info> {
    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Account<'info, Config>,

    #[account(address = config.rotator)]
    pub rotator: Signer<'info>,

    #[account(mut, seeds = [SEED_POOL], bump)]
    pub pool: Account<'info, Pool>,
}

pub fn handler(ctx: Context<Rotate>, delegate: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let pool = &mut ctx.accounts.pool;

    pool.rotate(config, delegate)?;

    Ok(())
}
