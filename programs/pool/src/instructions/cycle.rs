use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[account(delegate: Pubkey)]
pub struct Cycle<'info> {
    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Account<'info, Config>,

    #[account(address = config.cycler)]
    pub cycler: Signer<'info>,

    #[account(mut, seeds = [SEED_POOL], bump)]
    pub pool: Account<'info, Pool>,
}

pub fn handler(ctx: Context<Cycle>, delegate: Pubkey) -> Result<()> {
    let config = &mut ctx.accounts.config;
    let pool = &mut ctx.accounts.pool;

    pool.cycle(config, delegate)?;

    Ok(())
}
