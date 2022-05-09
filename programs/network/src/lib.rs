pub mod errors;
pub mod id;
pub mod pda;
pub mod state;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use instructions::*;

#[program]
pub mod cronos_network {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        config_bump: u8,
        pool_bump: u8,
        registry_bump: u8,
        snapshot_bump: u8,
    ) -> Result<()> {
        initialize::handler(ctx, config_bump, pool_bump, registry_bump, snapshot_bump)
    }

    pub fn register(ctx: Context<Register>, node_bump: u8) -> Result<()> {
        register::handler(ctx, node_bump)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        stake::handler(ctx, amount)
    }
}
