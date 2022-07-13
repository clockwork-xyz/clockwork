pub mod errors;
pub mod id;
pub mod state;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use instructions::*;
use state::HttpMethod;

#[program]
pub mod cronos_network {

    use super::*;

    pub fn initialize<'info>(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn request_ack<'info>(ctx: Context<RequestAck>) -> Result<()> {
        request_ack::handler(ctx)
    }

    pub fn request_new<'info>(
        ctx: Context<RequestNew>,
        ack_authority: Pubkey,
        method: HttpMethod,
        url: String,
    ) -> Result<()> {
        request_new::handler(ctx, ack_authority, method, url)
    }
}
