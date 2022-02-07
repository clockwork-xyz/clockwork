use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(
    authority_bump: u8,
    config_bump: u8,
    health_bump: u8,
    treasury_bump: u8,
)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [SEED_AUTHORITY],
        bump = authority_bump,
        payer = signer,
        space = 8 + size_of::<Authority>(),
    )]
    pub authority: Account<'info, Authority>,

    #[account(
        init,
        seeds = [SEED_CONFIG],
        bump = config_bump,
        payer = signer,
        space = 8 + size_of::<Config>(),
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        seeds = [SEED_HEALTH],
        bump = health_bump,
        payer = signer,
        space = 8 + size_of::<Health>(),
    )]
    pub health: Account<'info, Health>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(
        init,
        seeds = [SEED_TREASURY],
        bump = treasury_bump,
        payer = signer,
        space = 8 + size_of::<Treasury>(),
    )]
    pub treasury: Account<'info, Treasury>,
}

pub fn handler(
    ctx: Context<Initialize>,
    authority_bump: u8,
    config_bump: u8,
    health_bump: u8,
    treasury_bump: u8,
) -> ProgramResult {
    // Get accounts.
    let authority = &mut ctx.accounts.authority;
    let config = &mut ctx.accounts.config;
    let health = &mut ctx.accounts.health;
    let signer = &ctx.accounts.signer;
    let treasury = &mut ctx.accounts.treasury;

    // Initialize authority account.
    authority.bump = authority_bump;

    // Initialize health account.
    health.real_time = 0;
    health.target_time = 0;
    health.bump = health_bump;

    // Initialize config account.
    config.admin = signer.key();
    config.program_fee = 0;
    config.worker_fee = 0;
    config.bump = config_bump;

    // Initialize treasury account.
    treasury.bump = treasury_bump;

    return Ok(());
}
