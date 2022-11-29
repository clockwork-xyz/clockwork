use {
    crate::state::{
        Api, ApiAccount, Config, HttpMethod, Request, RequestAccount, SEED_REQUEST,
    },
    anchor_lang::{
        prelude::*,
        solana_program::system_program,
        system_program::{transfer, Transfer},
    },
    clockwork_network_program::state::Pool,
    std::{collections::HashMap, mem::size_of},
};

#[derive(Accounts)]
#[instruction(
    id: String, 
    method: HttpMethod, 
    route: String
)]
pub struct RequestNew<'info> {
    #[account(address = api.pubkey())]
    pub api: Account<'info, Api>,

    #[account()]
    pub caller: Signer<'info>,

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
            api.key().as_ref(),
            caller.key().as_ref(),
            id.as_bytes(),
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
    ctx: Context<RequestNew>,
    id: String,
    method: HttpMethod,
    route: String,
) -> Result<()> {
    // Fetch accounts
    let api = &ctx.accounts.api;
    let caller = &ctx.accounts.caller;
    let config = &ctx.accounts.config;
    let payer = &mut ctx.accounts.payer;
    let pool = &ctx.accounts.pool;
    let request = &mut ctx.accounts.request;
    let system_program = &ctx.accounts.system_program;

    // TODO Validate route is a relative path

    // Initialize the request account
    let current_slot = Clock::get().unwrap().slot;
    let fee_amount = config.request_fee;
    let headers = HashMap::new(); // TODO Get headers from ix data
    let workers = pool
        .clone()
        .into_inner()
        .workers
        .iter()
        .map(|k| *k)
        .collect::<Vec<Pubkey>>();
    request.init(
        api,
        caller.key(),
        current_slot,
        fee_amount,
        headers,
        id,
        method,
        route,
        workers,
    )?;

    // Transfer fees into request account to hold in escrow
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
