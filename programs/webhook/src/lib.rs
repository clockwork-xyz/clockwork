pub mod errors;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("E7p5KFo8kKCDm6BUnWtnVFkQSYh6ZA6xaGAuvpv8NXTa");

#[program]
pub mod webhook_program {
    pub use super::*;

    pub fn admin_config_update(
        ctx: Context<AdminConfigUpdate>,
        settings: ConfigSettings,
    ) -> Result<()> {
        admin_config_update::handler(ctx, settings)
    }

    pub fn initialize<'info>(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn request_ack<'info>(ctx: Context<RequestAck>) -> Result<()> {
        request_ack::handler(ctx)
    }

    pub fn request_create<'info>(
        ctx: Context<RequestCreate>,
        id: Vec<u8>,
        method: HttpMethod,
        route: String,
    ) -> Result<()> {
        request_new::handler(ctx, id, method, route)
    }
}
