use crate::state::*;

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(new_min_recurr: i64)]
pub struct AdminUpdateMinRecurr<'info> {
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

pub fn handler(ctx: Context<AdminUpdateMinRecurr>, new_min_recurr: i64) -> Result<()> {
    let admin = &ctx.accounts.admin;
    let config = &mut ctx.accounts.config;

    config.update_min_recurr(admin, new_min_recurr)
}
