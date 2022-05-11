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
        // authority_bump: u8,
        // config_bump: u8,
        // fee_bump: u8,
        // pool_bump: u8,
        // queue_bump: u8,
        // registry_bump: u8,
        // snapshot_bump: u8,
        // task_bump: u8,
    ) -> Result<()> {
        initialize::handler(
            ctx,
            // authority_bump,
            // config_bump,
            // fee_bump,
            // pool_bump,
            // queue_bump,
            // registry_bump,
            // snapshot_bump,
            // task_bump,
        )
    }

    pub fn register(ctx: Context<Register>, node_bump: u8) -> Result<()> {
        register::handler(ctx, node_bump)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        stake::handler(ctx, amount)
    }
}
