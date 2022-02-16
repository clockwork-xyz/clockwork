use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(
    new_worker_fee: u64,
)]
pub struct AdminUpdateWorkerFee<'info> {
    #[account(
        mut,
        address = config.admin,
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_CONFIG],
        bump = config.bump,
        owner = crate::ID,
    )]
    pub config: Account<'info, Config>,
}

pub fn handler(ctx: Context<AdminUpdateWorkerFee>, new_worker_fee: u64) -> ProgramResult {
    let config = &mut ctx.accounts.config;
    config.worker_fee = new_worker_fee;
    Ok(())
}
