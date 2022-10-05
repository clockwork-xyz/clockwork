//! This program allows users to create transaction queues on Solana. Queues are dynamic, long-running
//! transaction threads that can persist across blocks and even run indefinitely. Developers can use queues
//! to schedule transactions and automate smart-contracts without relying on centralized infrastructure.

pub mod errors;
pub mod objects;
pub mod utils;

mod instructions;

use anchor_lang::prelude::*;
use instructions::*;
use objects::*;

declare_id!("AfWZ9PyWW3E3G1TVYkKCe5w6BUC6w5Rim1DcmbAeaU3H");

/// Program for creating transaction queues on Solana.
#[program]
pub mod queue_program {
    use super::*;

    /// Admin instruction for updating the program config.
    pub fn config_update(ctx: Context<ConfigUpdate>, settings: ConfigSettings) -> Result<()> {
        config_update::handler(ctx, settings)
    }

    /// Admin instruction for initializing the program after deployment.
    pub fn initialize(ctx: Context<Initialize>, worker_pool: Pubkey) -> Result<()> {
        initialize::handler(ctx, worker_pool)
    }

    /// Cranks a transaction queue.
    pub fn queue_crank(ctx: Context<QueueCrank>, data_hash: Option<u64>) -> Result<()> {
        queue_crank::handler(ctx, data_hash)
    }

    /// Creates a new transaction queue.
    pub fn queue_create(
        ctx: Context<QueueCreate>,
        id: String,
        kickoff_instruction: InstructionData,
        trigger: Trigger,
    ) -> Result<()> {
        queue_create::handler(ctx, id, kickoff_instruction, trigger)
    }

    /// Closes an existing queue account and returns the lamports to the owner.
    pub fn queue_delete(ctx: Context<QueueDelete>) -> Result<()> {
        queue_delete::handler(ctx)
    }

    /// Pauses an active queue.
    pub fn queue_pause(ctx: Context<QueuePause>) -> Result<()> {
        queue_pause::handler(ctx)
    }

    /// Resumes a paused queue.
    pub fn queue_resume(ctx: Context<QueueResume>) -> Result<()> {
        queue_resume::handler(ctx)
    }

    /// Allows an owner to update the mutable properties of a queue.
    pub fn queue_update(
        ctx: Context<QueueUpdate>,
        kickoff_instruction: Option<InstructionData>,
        rate_limit: Option<u64>,
        trigger: Option<Trigger>,
    ) -> Result<()> {
        queue_update::handler(ctx, kickoff_instruction, rate_limit, trigger)
    }

    /// Allows an owner to withdraw from a queue's lamport balance.
    pub fn queue_withdraw(ctx: Context<QueueWithdraw>, amount: u64) -> Result<()> {
        queue_withdraw::handler(ctx, amount)
    }
}
