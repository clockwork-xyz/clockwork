use {crate::objects::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct PoolRotate<'info> {
    #[account(
        address = Config::pubkey(), 
        has_one = pool_authority
    )]
    pub config: Account<'info, Config>,

    #[account(mut, address = pool.pubkey())]
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
