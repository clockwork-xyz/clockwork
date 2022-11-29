//! This program allows users to create transaction threads on Solana. Threads are dynamic, long-running
//! transaction threads that can persist across blocks and even run indefinitely. Developers can use threads
//! to schedule transactions and automate smart-contracts without relying on centralized infrastructure.
#[macro_use]
extern crate version;

pub mod errors;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use clockwork_utils::CrateInfo;
use instructions::*;
use state::*;

declare_id!("3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv");

/// Program for creating transaction threads on Solana.
#[program]
pub mod thread_program {
    use super::*;

    /// Return the crate information via `sol_set_return_data/sol_get_return_data`
    pub fn get_crate_info(ctx: Context<GetCrateInfo>) -> Result<CrateInfo> {
        get_crate_info::handler(ctx)
    }

    /// Executes the next instruction on thread.
    pub fn thread_exec(ctx: Context<ThreadExec>) -> Result<()> {
        thread_exec::handler(ctx)
    }

    /// Creates a new transaction thread.
    pub fn thread_create(
        ctx: Context<ThreadCreate>,
        id: String,
        kickoff_instruction: InstructionData,
        trigger: Trigger,
    ) -> Result<()> {
        thread_create::handler(ctx, id, kickoff_instruction, trigger)
    }

    /// Closes an existing thread account and returns the lamports to the owner.
    pub fn thread_delete(ctx: Context<ThreadDelete>) -> Result<()> {
        thread_delete::handler(ctx)
    }

    /// Kicks off a thread if its trigger condition is active.
    pub fn thread_kickoff(ctx: Context<ThreadKickoff>) -> Result<()> {
        thread_kickoff::handler(ctx)
    }

    /// Pauses an active thread.
    pub fn thread_pause(ctx: Context<ThreadPause>) -> Result<()> {
        thread_pause::handler(ctx)
    }

    /// Resumes a paused thread.
    pub fn thread_resume(ctx: Context<ThreadResume>) -> Result<()> {
        thread_resume::handler(ctx)
    }

    /// Resumes a paused thread.
    pub fn thread_stop(ctx: Context<ThreadStop>) -> Result<()> {
        thread_stop::handler(ctx)
    }

    /// Allows an owner to update the mutable properties of a thread.
    pub fn thread_update(ctx: Context<ThreadUpdate>, settings: ThreadSettings) -> Result<()> {
        thread_update::handler(ctx, settings)
    }

    /// Allows an owner to withdraw from a thread's lamport balance.
    pub fn thread_withdraw(ctx: Context<ThreadWithdraw>, amount: u64) -> Result<()> {
        thread_withdraw::handler(ctx, amount)
    }
}
