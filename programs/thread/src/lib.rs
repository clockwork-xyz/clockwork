//! This program allows users to create transaction threads on Solana. Threads are dynamic, long-running
//! transaction threads that can persist across blocks and even run indefinitely. Developers can use threads
//! to schedule transactions and automate smart-contracts without relying on centralized infrastructure.
#[macro_use]
extern crate version;

pub mod errors;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use clockwork_utils::{
    thread::{SerializableInstruction, Trigger},
    CrateInfo,
};
use instructions::*;
use state::*;

declare_id!("CLoCKyJ6DXBJqqu2VWx9RLbgnwwR6BMHHuyasVmfMzBh");

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
        amount: u64,
        id: Vec<u8>,
        instructions: Vec<SerializableInstruction>,
        trigger: Trigger,
    ) -> Result<()> {
        thread_create::handler(ctx, amount, id, instructions, trigger)
    }

    /// Closes an existing thread account and returns the lamports to the owner.
    pub fn thread_delete(ctx: Context<ThreadDelete>) -> Result<()> {
        thread_delete::handler(ctx)
    }

    /// Appends a new instruction to the thread's instruction set.
    pub fn thread_instruction_add(
        ctx: Context<ThreadInstructionAdd>,
        instruction: SerializableInstruction,
    ) -> Result<()> {
        thread_instruction_add::handler(ctx, instruction)
    }

    /// Removes an instruction to the thread's instruction set at the provied index.
    pub fn thread_instruction_remove(
        ctx: Context<ThreadInstructionRemove>,
        index: u64,
    ) -> Result<()> {
        thread_instruction_remove::handler(ctx, index)
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

    /// Resets a thread's next instruction.
    pub fn thread_reset(ctx: Context<ThreadReset>) -> Result<()> {
        thread_reset::handler(ctx)
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
