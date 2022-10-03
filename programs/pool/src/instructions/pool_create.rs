use {
    crate::objects::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(name: String, size: usize)]
pub struct PoolCreate<'info> {
    #[account(
        address = Config::pubkey(), 
        has_one = pool_authority
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        address = Pool::pubkey(name),
        payer = payer,
        space = 8 + size_of::<Pool>() + (size_of::<Pubkey>() * size) + name.as_bytes().len(),
    )]
    pub pool: Account<'info, Pool>,

    #[account()]
    pub pool_authority: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PoolCreate>, name: String, size: usize) -> Result<()> {
    // Get accounts
    let pool = &mut ctx.accounts.pool;

    // Initialize the pool
    pool.init(name, size)?;

    Ok(())
}
