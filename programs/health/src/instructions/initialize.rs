use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [SEED_HEALTH],
        bump,
        payer = signer,
        space = 8 + size_of::<Health>(),
    )]
    pub health: Account<'info, Health>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    let health = &mut ctx.accounts.health;

    health.new()?;

    // TODO create a queue
    // TODO create an task to call ping every X seconds

    Ok(())
}
