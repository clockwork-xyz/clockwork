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
            base_url.as_bytes().as_ref(),
            owner.key().as_ref(),
        ],
        bump,
        payer = owner,
        space = 8 + size_of::<Api>() + base_url.len(),
    )]
    pub api: Account<'info, Api>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(ctx: Context<ApiNew>, base_url: String) -> Result<()> {
    // Get accounts
    let ack_authority = &ctx.accounts.ack_authority;
    let api = &mut ctx.accounts.api;
    let owner = &mut ctx.accounts.owner;

    // TODO Validate base_url

    // Initialize the api account
    api.new(ack_authority.key(), base_url, owner.key())?;

    Ok(())
}
