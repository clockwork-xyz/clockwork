pub mod errors;
pub mod id;
pub mod state;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use instructions::*;

#[program]
pub mod cronos_network {

    use super::*;

    pub fn initialize<'info>(ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn request_new<'info>(ctx: Context<'_, '_, '_, 'info, RequestNew<'info>>) -> Result<()> {
        request_new::handler(ctx)
    }
}
