pub mod errors;
pub mod pda;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("5Xu6iNDMf17wVC6aSoKeQsjF87aTQjrqpA9sX3sA5VJX");

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
