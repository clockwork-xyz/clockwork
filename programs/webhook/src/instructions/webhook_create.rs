use std::{collections::HashMap, mem::size_of};

use anchor_lang::{
    prelude::*,
    solana_program::system_program,
    system_program::{transfer, Transfer},
};

use crate::state::{Relayer, HttpMethod, Webhook, SEED_WEBHOOK};

static WEBHOOK_FEE: u64 = 1_000_000; 

#[derive(Accounts)]
#[instruction(
    body: Vec<u8>,
    headers: HashMap<String, String>,
    id: Vec<u8>, 
    method: HttpMethod, 
    url: String
)]
pub struct WebhookCreate<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_WEBHOOK,
            authority.key().as_ref(),
            id.as_slice(),
        ],
        bump,
        space = 8 + size_of::<Webhook>(),
        payer = payer
    )]
    pub webhook: Account<'info, Webhook>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler<'info>(
    ctx: Context<WebhookCreate>,
    body: Vec<u8>,
    headers: HashMap<String, String>,
    id: Vec<u8>,
    method: HttpMethod,
    url: String,
) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let payer = &mut ctx.accounts.payer;
    let webhook = &mut ctx.accounts.webhook;
    let system_program = &ctx.accounts.system_program;

    // Initialize the webhook account
    let current_slot = Clock::get().unwrap().slot;
    webhook.authority = authority.key();
    webhook.body = body;
    webhook.created_at = current_slot;
    webhook.headers = headers;
    webhook.id = id;
    webhook.method = method;
    webhook.relayer = Relayer::Clockwork;
    webhook.url = url;

    // Transfer fees into webhook account to hold in escrow.
    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: payer.to_account_info(),
                to: webhook.to_account_info(),
            },
        ),
        WEBHOOK_FEE,
    )?;

    Ok(())
}
