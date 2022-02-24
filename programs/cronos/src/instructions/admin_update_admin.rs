use crate::state::*;

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(new_admin: Pubkey)]
pub struct AdminUpdateAdmin<'info> {
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

pub fn handler(ctx: Context<AdminUpdateAdmin>, new_admin: Pubkey) -> Result<()> {
    let admin = &ctx.accounts.admin;
    let config = &mut ctx.accounts.config;

    config.update_admin(admin, new_admin)
}
