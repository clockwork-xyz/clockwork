use crate::state::{HttpMethod, Request, RequestAccount, SEED_REQUEST};
use anchor_lang::{prelude::*, solana_program::system_program};
use std::{collections::HashMap, mem::size_of};

#[derive(Accounts)]
#[instruction(
    method: HttpMethod, 
    url: String
)]
pub struct RequestNew<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [SEED_REQUEST],
        bump,
        space = 8 + size_of::<Request>(),
        payer = payer
    )]
    pub request: Account<'info, Request>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(ctx: Context<RequestNew>, method: HttpMethod, url: String) -> Result<()> {
    // Fetch accounts
    let payer = &mut ctx.accounts.payer;
    let request = &mut ctx.accounts.request;

    // Initialize the request account
    let headers = HashMap::new();
    request.new(
        headers,
        method,
        payer.key(),
        url,
    )?;

    Ok(())
}
