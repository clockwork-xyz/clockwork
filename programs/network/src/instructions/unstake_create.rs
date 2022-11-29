use {
    crate::{errors::*, state::*}, 
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct UnstakeCreate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [
            SEED_DELEGATION,
            delegation.worker.as_ref(),
            delegation.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = authority,
        has_one = worker,
    )]
    pub delegation: Account<'info, Delegation>,

    #[account(
        mut,
        seeds = [SEED_REGISTRY],
        bump, 
        constraint = !registry.locked @ ClockworkError::RegistryLocked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(
        init,
        seeds = [
            SEED_UNSTAKE,
            registry.total_unstakes.to_be_bytes().as_ref(),
        ],
        bump,
        payer = authority,
        space = 8 + size_of::<Unstake>(),
    )]
    pub unstake: Account<'info, Unstake>,

    #[account(address = worker.pubkey())]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<UnstakeCreate>, amount: u64) -> Result<()> {
    // Get accounts.
    let authority = &ctx.accounts.authority;
    let delegation = &ctx.accounts.delegation;
    let registry = &mut ctx.accounts.registry;
    let unstake = &mut ctx.accounts.unstake;
    let worker = &ctx.accounts.worker;

    // Validate the request is valid.
    require!(amount.le(&delegation.stake_amount), ClockworkError::InvalidUnstakeAmount);

    // Initialize the unstake account.
    unstake.init(amount, authority.key(), delegation.key(), registry.total_unstakes, worker.key())?;

    // Increment the registry's unstake counter.
    registry.total_unstakes = registry.total_unstakes.checked_add(1).unwrap();

    Ok(())
}
