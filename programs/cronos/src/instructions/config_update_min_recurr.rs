use {crate::errors::*, crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(
    new_min_recurr: i64,
)]
pub struct ConfigUpdateMinRecurr<'info> {
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

pub fn handler(ctx: Context<ConfigUpdateMinRecurr>, new_min_recurr: i64) -> ProgramResult {
    let config = &mut ctx.accounts.config;
    require!(new_min_recurr > 0, ErrorCode::InvalidRecurrNegative);
    config.min_recurr = new_min_recurr;
    Ok(())
}
