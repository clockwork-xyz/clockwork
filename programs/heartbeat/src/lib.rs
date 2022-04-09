pub mod errors;
pub mod pda;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("B9d3x3owH4F3cX8fF18EVvr35xpRsACa5xJgDzEeVgsN");

#[program]
pub mod cronos_heartbeat {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, config_bump: u8, heartbeat_bump: u8) -> Result<()> {
        initialize::handler(ctx, config_bump, heartbeat_bump)
    }

    pub fn ping(ctx: Context<Ping>) -> Result<()> {
        ping::handler(ctx)
    }

    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        reset::handler(ctx)
    }
}
