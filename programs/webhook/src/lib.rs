pub mod errors;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("E7p5KFo8kKCDm6BUnWtnVFkQSYh6ZA6xaGAuvpv8NXTa");

#[program]
pub mod webhook_program {
    use super::*;

    pub fn admin_config_update(
        ctx: Context<AdminConfigUpdate>,
        settings: ConfigSettings,
    ) -> Result<()> {
        admin_config_update::handler(ctx, settings)
    }

    pub fn admin_fee_claim<'info>(ctx: Context<AdminFeeClaim>, amount: u64) -> Result<()> {
        admin_fee_claim::handler(ctx, amount)
    }

    pub fn api_new<'info>(ctx: Context<ApiNew>, base_url: String) -> Result<()> {
        api_new::handler(ctx, base_url)
    }

    pub fn fee_claim<'info>(ctx: Context<FeeClaim>, amount: u64) -> Result<()> {
        fee_claim::handler(ctx, amount)
    }

    pub fn initialize<'info>(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn request_ack<'info>(ctx: Context<RequestAck>) -> Result<()> {
        request_ack::handler(ctx)
    }

    pub fn request_new<'info>(
        ctx: Context<RequestNew>,
        id: String,
        method: HttpMethod,
        route: String,
    ) -> Result<()> {
        request_new::handler(ctx, id, method, route)
    }
}
