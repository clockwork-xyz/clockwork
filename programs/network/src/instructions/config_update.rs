use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(settings: ConfigSettings)]
pub struct ConfigUpdate<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut, 
        seeds = [SEED_CONFIG],
        bump, 
        has_one = admin
    )]
    pub config: Account<'info, Config>,
}

pub fn handler(ctx: Context<ConfigUpdate>, settings: ConfigSettings) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.update(settings)
}
