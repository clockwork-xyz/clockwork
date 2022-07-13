use {
    crate::state::{Config, HttpMethod, Manager, ManagerAccount, Request, RequestAccount, SEED_CONFIG, SEED_MANAGER, SEED_REQUEST},
    anchor_lang::{prelude::*, solana_program::{system_program, sysvar}, system_program::{transfer, Transfer}},
    std::{collections::HashMap, mem::size_of},
};

#[derive(Accounts)]
#[instruction(
    ack_authority: Pubkey,
    method: HttpMethod, 
    url: String,
)]
pub struct RequestNew<'info> {
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Account<'info, Config>,

    #[account(
        init_if_needed,
        seeds = [
            SEED_MANAGER,
            payer.key().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Manager>(),
    )]
    pub manager: Account<'info, Manager>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_REQUEST,
            manager.key().as_ref(),
            manager.request_count.to_be_bytes().as_ref(),
        ],
        bump,
        space = 8 + size_of::<Request>(),
        payer = payer
    )]
    pub request: Account<'info, Request>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(ctx: Context<RequestNew>, ack_authority: Pubkey, method: HttpMethod, url: String) -> Result<()> {
    // Fetch accounts
    let clock = &ctx.accounts.clock;
    let config = &ctx.accounts.config;
    let manager = &mut ctx.accounts.manager;
    let payer = &mut ctx.accounts.payer;
    let request = &mut ctx.accounts.request;
    let system_program = &ctx.accounts.system_program;

    // Initialize the manager account if needed
    if manager.clone().into_inner().request_count == 0 {
        manager.new(payer.key())?;
    }

    // Initialize the request account
    let created_at = clock.slot;
    let fee_amount = config.request_fee;
    let headers = HashMap::new(); // TODO Get headers from ix data
    request.new(
        ack_authority,
        created_at,
        fee_amount,
        headers,
        manager,
        method,
        url,
    )?;

    // Transfer fees into request account to hold in escrow
    transfer(
        CpiContext::new(
            system_program.to_account_info(), 
            Transfer {
                from: payer.to_account_info(),
                to: request.to_account_info(),
            }
        ), 
        fee_amount
    )?;

    Ok(())
}
