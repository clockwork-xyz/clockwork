use {
    crate::state::*, 
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of
};

#[derive(Accounts)]
pub struct DelegateNew<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_DELEGATE, 
            authority.key().as_ref()
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Delegate>(),
    )]
    pub delegate: Account<'info, Delegate>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DelegateNew>) -> Result<()> {
    let authority = &ctx.accounts.authority;
    let delegate = &mut ctx.accounts.delegate;

    delegate.new(authority.key())?;

    Ok(())
}
