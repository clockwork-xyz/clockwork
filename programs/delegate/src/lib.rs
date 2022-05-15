pub mod errors;
pub mod id;
pub mod pda;
pub mod state;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use instructions::*;

#[program]
pub mod cronos_delegate {
    use super::*;

    // pub fn pool_cycle(ctx: Context<HeartbeatPing>) -> Result<()> {
    //     heartbeat_ping::handler(ctx)
    // }

    pub fn initialize(ctx: Context<Initialize>, authorized_cycler: Pubkey) -> Result<()> {
        initialize::handler(ctx, authorized_cycler)
    }
}
