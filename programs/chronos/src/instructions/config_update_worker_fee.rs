use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(
    new_worker_fee: u64,
)]
pub struct ConfigUpdateWorkerFee<'info> {
    #[account(
        mut,
        seeds = [SEED_CONFIG],
        bump = config.bump,
        owner = crate::ID,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        address = config.admin_authority,
    )]
    pub signer: Signer<'info>,
}

pub fn handler(ctx: Context<ConfigUpdateWorkerFee>, new_worker_fee: u64) -> ProgramResult {
    let config = &mut ctx.accounts.config;
    config.worker_fee = new_worker_fee;
    Ok(())
}
