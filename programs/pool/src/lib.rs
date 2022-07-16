pub mod errors;
pub mod id;
pub mod state;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use instructions::*;

#[program]
pub mod cronos_pool {
    use super::*;

    pub fn rotate(ctx: Context<Rotate>, delegate: Pubkey) -> Result<()> {
        rotate::handler(ctx, delegate)
    }

    pub fn initialize(ctx: Context<Initialize>, rotator: Pubkey) -> Result<()> {
        initialize::handler(ctx, rotator)
    }
}
