pub mod anchor;
pub mod errors;
pub mod id;
pub mod payer;
pub mod state;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

#[program]
pub mod clockwork_crank {
    use super::*;

    pub fn queue_crank(ctx: Context<QueueCrank>) -> Result<()> {
        queue_crank::handler(ctx)
    }

    pub fn queue_create(
        ctx: Context<QueueCreate>,
        balance: u64,
        instruction: InstructionData,
        name: String,
        trigger: Trigger,
    ) -> Result<()> {
        queue_create::handler(ctx, balance, instruction, name, trigger)
    }

    pub fn queue_start(ctx: Context<QueueStart>) -> Result<()> {
        queue_start::handler(ctx)
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
