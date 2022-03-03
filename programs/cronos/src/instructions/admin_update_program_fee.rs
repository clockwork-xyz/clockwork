use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(new_program_fee: u64)]
pub struct AdminUpdateProgramFee<'info> {
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

pub fn handler(ctx: Context<AdminUpdateProgramFee>, new_program_fee: u64) -> Result<()> {
    let admin = &ctx.accounts.admin;
    let config = &mut ctx.accounts.config;

    config.update_program_fee(admin, new_program_fee)
}
