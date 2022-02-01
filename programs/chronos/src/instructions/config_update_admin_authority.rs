use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(
    new_admin_authority: Pubkey,
)]
pub struct ConfigUpdateAdminAuthority<'info> {
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

pub fn handler(
    ctx: Context<ConfigUpdateAdminAuthority>,
    new_admin_authority: Pubkey,
) -> ProgramResult {
    let config = &mut ctx.accounts.config;
    config.admin_authority = new_admin_authority;
    Ok(())
}
