pub mod errors;
pub mod pda;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("2ysCzNg41B2FSnjHqcyRVdrzf9d2pazqLARrb2qDJpbF");

#[program]
pub mod cronos_heartbeat {
    use super::*;

    pub fn heartbeat_ping(ctx: Context<HeartbeatPing>) -> Result<()> {
        heartbeat_ping::handler(ctx)
    }

    pub fn heartbeat_reset(ctx: Context<HeartbeatReset>) -> Result<()> {
        heartbeat_reset::handler(ctx)
    }

    pub fn initialize(ctx: Context<Initialize>, config_bump: u8, heartbeat_bump: u8) -> Result<()> {
        initialize::handler(ctx, config_bump, heartbeat_bump)
    }
}
