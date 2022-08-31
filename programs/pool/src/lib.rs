pub mod errors;
pub mod id;
pub mod state;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

#[program]
pub mod clockwork_pool {
    use super::*;

    pub fn config_update(ctx: Context<ConfigUpdate>, settings: ConfigSettings) -> Result<()> {
        config_update::handler(ctx, settings)
    }

    pub fn initialize(ctx: Context<Initialize>, pool_authority: Pubkey) -> Result<()> {
        initialize::handler(ctx, pool_authority)
    }

    pub fn pool_create(ctx: Context<PoolCreate>, name: String, size: usize) -> Result<()> {
        pool_create::handler(ctx, name, size)
    }

    pub fn pool_rotate(ctx: Context<PoolRotate>) -> Result<()> {
        pool_rotate::handler(ctx)
    }

    pub fn pool_update(ctx: Context<PoolUpdate>, settings: PoolSettings) -> Result<()> {
        pool_update::handler(ctx, settings)
    }
}
