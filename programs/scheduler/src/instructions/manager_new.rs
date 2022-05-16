use {
    crate::state::*, 
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of
};

#[derive(Accounts)]
pub struct ManagerNew<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_MANAGER, 
            authority.key().as_ref()
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Manager>(),
    )]
    pub manager: Account<'info, Manager>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ManagerNew>) -> Result<()> {
    let authority = &ctx.accounts.authority;
    let manager = &mut ctx.accounts.manager;

    manager.new(authority.key())?;

    Ok(())
}
