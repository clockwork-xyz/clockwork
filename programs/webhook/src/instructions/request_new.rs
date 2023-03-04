use std::{collections::HashMap, mem::size_of};

use anchor_lang::{
    prelude::*,
    solana_program::system_program,
    system_program::{transfer, Transfer},
};
use clockwork_network_program::state::Pool;

use crate::state::{Relayer, Config, HttpMethod, Request, SEED_REQUEST};

#[derive(Accounts)]
#[instruction(
    id: Vec<u8>, 
    method: HttpMethod, 
    url: String
)]
pub struct RequestCreate<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account()]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        seeds = [
            SEED_REQUEST,
            authority.key().as_ref(),
            id.as_slice(),
        ],
        bump,
        space = 8 + size_of::<Request>(),
        payer = payer
    )]
    pub request: Account<'info, Request>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(
    ctx: Context<RequestCreate>,
    id: Vec<u8>,
    method: HttpMethod,
    url: String,
) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let config = &ctx.accounts.config;
    let payer = &mut ctx.accounts.payer;
    let pool = &ctx.accounts.pool;
    let request = &mut ctx.accounts.request;
    let system_program = &ctx.accounts.system_program;

    // Initialize the request account
    let current_slot = Clock::get().unwrap().slot;
    let fee_amount = config.request_fee;
    let headers = HashMap::new(); // TODO Get headers from ix data
    request.authority = authority.key();
    request.created_at = current_slot;
    request.headers = headers;
    request.id = id;
    request.method = method;
    request.relayer = Relayer::Clockwork;
    request.url = url;
    request.workers = pool
        .clone()
        .into_inner()
        .workers
        .iter()
        .map(|k| *k)
        .collect::<Vec<Pubkey>>();

    // Transfer fees into request account to hold in escrow.
    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: payer.to_account_info(),
                to: request.to_account_info(),
            },
        ),
        fee_amount,
    )?;

    Ok(())
}
