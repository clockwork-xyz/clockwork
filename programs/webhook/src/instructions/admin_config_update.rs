use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(settings: ConfigSettings)]
pub struct AdminConfigUpdate<'info> {
    #[account(mut, address = config.admin)]
    pub admin: Signer<'info>,

    #[account(mut, seeds = [SEED_CONFIG], bump)]
    pub config: Account<'info, Config>,
}

pub fn handler(ctx: Context<AdminConfigUpdate>, settings: ConfigSettings) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.update(settings)
}
