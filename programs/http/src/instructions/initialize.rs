use anchor_lang::{prelude::*, solana_program::system_program};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(_ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
    // TODO
    Ok(())
}
