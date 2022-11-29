use {
    crate::state::{Api, ApiAccount, SEED_API},
    anchor_lang::{prelude::*, system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(base_url: String)]
pub struct ApiNew<'info> {
    #[account()]
    pub ack_authority: SystemAccount<'info>,

    #[account(
        init,
        seeds = [
            SEED_API,
            authority.key().as_ref(),
            base_url.as_bytes(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Api>() + base_url.len(),
    )]
    pub api: Account<'info, Api>,

    #[account()]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(ctx: Context<ApiNew>, base_url: String) -> Result<()> {
    // Get accounts
    let ack_authority = &ctx.accounts.ack_authority;
    let authority = &mut ctx.accounts.authority;
    let api = &mut ctx.accounts.api;

    // TODO Validate base_url

    // Initialize the api account
    api.init(ack_authority.key(), authority.key(), base_url)?;

    Ok(())
}
