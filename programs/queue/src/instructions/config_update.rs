use {crate::state::*, anchor_lang::prelude::*};

/// Required accounts for the `config_update` instruction.
#[derive(Accounts)]
#[instruction(settings: ConfigSettings)]
pub struct ConfigUpdate<'info> {
    /// The program admin.
    #[account(mut)]
    pub admin: Signer<'info>,

    /// The program config account.
    #[account(mut, seeds = [SEED_CONFIG], bump, has_one = admin)]
    pub config: Account<'info, Config>,
}

pub fn handler(ctx: Context<ConfigUpdate>, settings: ConfigSettings) -> Result<()> {
    let config = &mut ctx.accounts.config;
    config.update(settings)
}
