pub mod errors;
pub mod id;
pub mod pda;
pub mod state;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use instructions::*;

#[program]
pub mod cronos_health {
    use super::*;

    pub fn ping(ctx: Context<Ping>) -> Result<()> {
        ping::handler(ctx)
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
