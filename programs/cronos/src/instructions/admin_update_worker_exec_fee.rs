use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(new_worker_exec_fee: u64)]
pub struct AdminUpdateWorkerExecFee<'info> {
    #[account(
        mut,
        address = config.admin,
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_CONFIG],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,
}

pub fn handler(ctx: Context<AdminUpdateWorkerExecFee>, new_worker_exec_fee: u64) -> Result<()> {
    let admin = &ctx.accounts.admin;
    let config = &mut ctx.accounts.config;

    config.update_worker_exec_fee(admin, new_worker_exec_fee)
}
