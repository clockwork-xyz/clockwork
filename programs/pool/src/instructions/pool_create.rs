use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(name: String, size: usize)]
pub struct PoolCreate<'info> {
    // TODO Verify this signer is a Clockwork PDA
    #[account()]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [SEED_POOL, name.as_bytes()],
        bump,
        payer = payer,
        space = 8 + size_of::<Pool>() + (size_of::<Pubkey>() * size) + name.as_bytes().len(),
    )]
    pub pool: Account<'info, Pool>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PoolCreate>, name: String, size: usize) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let pool = &mut ctx.accounts.pool;

    // Initialize the pool
    pool.init(authority.key(), name, size)?;

    Ok(())
}
