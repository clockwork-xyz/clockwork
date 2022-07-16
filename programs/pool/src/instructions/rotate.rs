use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct Rotate<'info> {
    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Account<'info, Config>,

    #[account(address = config.rotator)]
    pub rotator: Signer<'info>,

    #[account(mut, seeds = [SEED_POOL], bump)]
    pub pool: Account<'info, Pool>,

    #[account()]
    pub worker: SystemAccount<'info>,
}

pub fn handler(ctx: Context<Rotate>) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let pool = &mut ctx.accounts.pool;
    let worker = &ctx.accounts.worker;

    pool.rotate(config, worker.key())?;

    Ok(())
}
