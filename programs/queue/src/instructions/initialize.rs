use {
    crate::objects::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

/// Accounts required by the `initialize` instruction.
#[derive(Accounts)]
#[instruction(worker_pool: Pubkey)]
pub struct Initialize<'info> {
    /// The program admin.
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The program config account.
    #[account(
        init,
        seeds = [SEED_CONFIG],
        bump,
        payer = admin,
        space = 8 + size_of::<Config>(),
    )]
    pub config: Account<'info, Config>,

    /// The Solana system program.
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, worker_pool: Pubkey) -> Result<()> {
    // Get accounts
    let admin = &ctx.accounts.admin;
    let config = &mut ctx.accounts.config;

    // Initialize config account
    config.init(admin.key(), worker_pool)?;

    Ok(())
}
