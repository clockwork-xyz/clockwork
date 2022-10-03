use {
    crate::objects::*,
    anchor_lang::{
        prelude::*,
        solana_program::system_program,
        system_program::{transfer, Transfer},
    },
    clockwork_pool_program::{objects::Pool, program::PoolProgram},
};

#[derive(Accounts)]
#[instruction(name: String, size: usize)]
pub struct PoolCreate<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(address = Config::pubkey(), has_one = admin)]
    pub config: Account<'info, Config>,

    #[account(address = Pool::pubkey(name))]
    pub pool: SystemAccount<'info>,

    #[account(address = clockwork_pool_program::ID)]
    pub pool_program: Program<'info, PoolProgram>,

    #[account(address = clockwork_pool_program::objects::Config::pubkey())]
    pub pool_program_config: Account<'info, clockwork_pool_program::objects::Config>,

    #[account(mut, address = Rotator::pubkey())]
    pub rotator: Account<'info, Rotator>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<PoolCreate>, name: String, size: usize) -> Result<()> {
    // Get accounts
    let admin = &ctx.accounts.admin;
    let pool = &ctx.accounts.pool;
    let pool_program = &ctx.accounts.pool_program;
    let pool_program_config = &ctx.accounts.pool_program_config;
    let rotator = &mut ctx.accounts.rotator;
    let system_program = &ctx.accounts.system_program;

    // Rotate the worker into its supported pools
    let rotator_bump = *ctx.bumps.get("rotator").unwrap();
    clockwork_pool_program::cpi::pool_create(
        CpiContext::new_with_signer(
            pool_program.to_account_info(),
            clockwork_pool_program::cpi::accounts::PoolCreate {
                config: pool_program_config.to_account_info(),
                payer: admin.to_account_info(),
                pool: pool.to_account_info(),
                pool_authority: rotator.to_account_info(),
                system_program: system_program.to_account_info(),
            },
            &[&[SEED_ROTATOR, &[rotator_bump]]],
        ),
        name,
        size,
    )?;

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
