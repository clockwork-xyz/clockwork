use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct PoolRotate<'info> {
    #[account(address = pool.authority)]
    pub authority: Signer<'info>,

    #[account(mut, seeds = [SEED_POOL, pool.name.as_bytes()], bump)]
    pub pool: Account<'info, Pool>,

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
