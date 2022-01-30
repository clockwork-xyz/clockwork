use {
    crate::state,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(
    authority_bump: u8,
)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        seeds = [state::SEED_AUTHORITY],
        bump = authority_bump,
        payer = signer,
        space = 8 + size_of::<state::Authority>(),
    )]
    pub authority: Account<'info, state::Authority>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, authority_bump: u8) -> ProgramResult {
    let authority = &mut ctx.accounts.authority;
    authority.bump = authority_bump;
    return Ok(());
}
