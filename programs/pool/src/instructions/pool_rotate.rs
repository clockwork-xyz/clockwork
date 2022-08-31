use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct PoolRotate<'info> {
    #[account(seeds = [SEED_CONFIG], bump, has_one = pool_authority)]
    pub config: Account<'info, Config>,

    #[account(mut, seeds = [SEED_POOL, pool.name.as_bytes()], bump)]
    pub pool: Account<'info, Pool>,

    #[account()]
    pub pool_authority: Signer<'info>,

    #[account()]
    pub worker: SystemAccount<'info>,
}

pub fn handler(ctx: Context<PoolRotate>) -> Result<()> {
    // Get accounts
    let pool = &mut ctx.accounts.pool;
    let worker = &ctx.accounts.worker;

    // Rotate the worker into the pool
    pool.rotate(worker.key())?;

    Ok(())
}
