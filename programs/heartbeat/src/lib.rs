pub mod errors;
pub mod pda;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("H7HagBMAbQCqSH6w7L5pmEGxo2WPdZpP81sjEjr2X6HV");

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
