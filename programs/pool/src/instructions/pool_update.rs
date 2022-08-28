use {
    crate::state::*,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(settings: PoolSettings)]
pub struct PoolUpdate<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(
        mut, 
        seeds = [SEED_POOL, pool.name.as_bytes()], 
        bump,
        has_one = authority
    )]
    pub pool: Account<'info, Pool>,
}

pub fn handler(ctx: Context<PoolUpdate>, settings: PoolSettings) -> Result<()> {
    // Get accounts
    let pool = &mut ctx.accounts.pool;

    // Update the pool settings
    pool.update(settings)?;

    Ok(())
}
