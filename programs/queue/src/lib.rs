pub mod errors;
pub mod id;
pub mod state;
pub mod utils;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

#[program]
pub mod queue_program {
    use super::*;

    pub fn config_update(ctx: Context<ConfigUpdate>, settings: ConfigSettings) -> Result<()> {
        config_update::handler(ctx, settings)
    }

    pub fn initialize(ctx: Context<Initialize>, worker_pool: Pubkey) -> Result<()> {
        initialize::handler(ctx, worker_pool)
    }

    pub fn queue_crank(ctx: Context<QueueCrank>, data_hash: Option<u64>) -> Result<()> {
        queue_crank::handler(ctx, data_hash)
    }

    pub fn queue_create(
        ctx: Context<QueueCreate>,
        id: String,
        kickoff_instruction: InstructionData,
        trigger: Trigger,
    ) -> Result<()> {
        queue_create::handler(ctx, id, kickoff_instruction, trigger)
    }

    pub fn queue_delete(ctx: Context<QueueDelete>) -> Result<()> {
        queue_delete::handler(ctx)
    }

    pub fn queue_pause(ctx: Context<QueuePause>) -> Result<()> {
        queue_pause::handler(ctx)
    }

    pub fn queue_resume(ctx: Context<QueueResume>) -> Result<()> {
        queue_resume::handler(ctx)
    }

    pub fn queue_update(
        ctx: Context<QueueUpdate>,
        kickoff_instruction: Option<InstructionData>,
        rate_limit: Option<u64>,
        trigger: Option<Trigger>,
    ) -> Result<()> {
        queue_update::handler(ctx, kickoff_instruction, rate_limit, trigger)
    }

    pub fn queue_withdraw(ctx: Context<QueueWithdraw>, amount: u64) -> Result<()> {
        queue_withdraw::handler(ctx, amount)
    }
}
