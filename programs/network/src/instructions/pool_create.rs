use {
    crate::objects::*,
    anchor_lang::{prelude::*, system_program::{transfer, Transfer},  solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(name: String, size: usize)]
pub struct PoolCreate<'info> {
    #[account(mut, address = config.admin)]
    pub admin: Signer<'info>,

    #[account(
        address = Config::pubkey(), 
        has_one = admin
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_POOL,
            name.as_bytes(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Pool>() + (size_of::<Pubkey>() * size) + name.as_bytes().len(),
    )]
    pub pool: Account<'info, Pool>,

    #[account()]
    pub pool_authority: Signer<'info>,

    #[account(mut, seeds = [SEED_ROTATOR], bump)]
    pub rotator: Account<'info, Rotator>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PoolCreate>, name: String, size: usize) -> Result<()> {
    // Get accounts
    let admin = &ctx.accounts.admin;
    let pool = &mut ctx.accounts.pool;
    let rotator = &mut ctx.accounts.rotator;
    let system_program = &ctx.accounts.system_program;

    // Initialize the pool
    pool.init(name, size)?;

    // Add new pool pubkey to the rotator
    rotator.add_pool(pool.key())?;

    // Realloc memory for the rotator account
    let data_len = 8 + rotator.try_to_vec()?.len();
    rotator.to_account_info().realloc(data_len, false)?;

    // If lamports are required to maintain rent-exemption, pay them
    let minimum_rent = Rent::get().unwrap().minimum_balance(data_len);
    if minimum_rent > rotator.to_account_info().lamports() {
        transfer(
            CpiContext::new(
                system_program.to_account_info(),
                Transfer {
                    from: admin.to_account_info(),
                    to: rotator.to_account_info(),
                },
            ),
            minimum_rent
                .checked_sub(rotator.to_account_info().lamports())
                .unwrap(),
        )?;
    }

    Ok(())
}
