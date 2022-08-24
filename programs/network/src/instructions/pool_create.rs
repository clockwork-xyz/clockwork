use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    clockwork_pool::state::SEED_POOL
};

#[derive(Accounts)]
#[instruction(name: String, size: usize)]
pub struct PoolCreate<'info> {
    #[account()]
    pub admin: Signer<'info>,

    #[account(seeds = [SEED_CONFIG], bump, has_one = admin)]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [
            SEED_POOL,
            name.as_bytes(),
        ], 
        bump,
        seeds::program = clockwork_pool::ID
    )]
    pub pool: SystemAccount<'info>,

    #[account(address = clockwork_pool::ID)]
    pub pool_program: Program<'info, clockwork_pool::program::ClockworkPool>,

    #[account(mut, seeds = [SEED_ROTATOR], bump)]
    pub rotator: Account<'info, Rotator>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PoolCreate>, name: String, size: usize) -> Result<()> {
    // Get accounts
    let pool = &ctx.accounts.pool;
    let pool_program = &ctx.accounts.pool_program;
    let rotator = &mut ctx.accounts.rotator;
    let system_program = &ctx.accounts.system_program;

    // Rotate the worker into its supported pools
    let rotator_bump = *ctx.bumps.get("rotator").unwrap();
    clockwork_pool::cpi::pool_create(
        CpiContext::new_with_signer(
            pool_program.to_account_info(),
            clockwork_pool::cpi::accounts::PoolCreate {
                authority: rotator.to_account_info(),
                pool: pool.to_account_info(),
                system_program: system_program.to_account_info(),
            },
            &[&[SEED_ROTATOR, &[rotator_bump]]],
        ),
        name,
        size,
    )?;

    // Add pool to the rotator
    rotator.add_pool(pool.key())?;

    Ok(())
}
