use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(
    new_admin: Pubkey,
)]
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
        owner = crate::ID,
    )]
    pub config: Account<'info, Config>,
}

pub fn handler(ctx: Context<AdminUpdateAdmin>, new_admin: Pubkey) -> ProgramResult {
    let config = &mut ctx.accounts.config;
    config.admin = new_admin;
    Ok(())
}
