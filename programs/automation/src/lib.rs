//! This program allows users to create transaction automations on Solana. Automations are dynamic, long-running
//! transaction automations that can persist across blocks and even run indefinitely. Developers can use automations
//! to schedule transactions and automate smart-contracts without relying on centralized infrastructure.
#[macro_use]
extern crate version;

pub mod errors;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use clockwork_utils::{
    automation::{InstructionData, Trigger},
    CrateInfo,
};
use instructions::*;
use state::*;

declare_id!("auto5LqrhPVVt34PDu3dPwJhRisGoFA6dYpxRn29n1k");

/// Program for creating transaction automations on Solana.
#[program]
pub mod automation_program {
    use super::*;

    /// Return the crate information via `sol_set_return_data/sol_get_return_data`
    pub fn get_crate_info(ctx: Context<GetCrateInfo>) -> Result<CrateInfo> {
        get_crate_info::handler(ctx)
    }

    /// Executes the next instruction on automation.
    pub fn automation_exec(ctx: Context<AutomationExec>) -> Result<()> {
        automation_exec::handler(ctx)
    }

    /// Creates a new transaction automation.
    pub fn automation_create(
        ctx: Context<AutomationCreate>,
        amount: u64,
        id: Vec<u8>,
        instructions: Vec<InstructionData>,
        trigger: Trigger,
    ) -> Result<()> {
        automation_create::handler(ctx, amount, id, instructions, trigger)
    }

    /// Closes an existing automation account and returns the lamports to the owner.
    pub fn automation_delete(ctx: Context<AutomationDelete>) -> Result<()> {
        automation_delete::handler(ctx)
    }

    /// Kicks off an automation if its trigger condition is active.
    pub fn automation_kickoff(ctx: Context<AutomationKickoff>) -> Result<()> {
        automation_kickoff::handler(ctx)
    }

    /// Pauses an active automation.
    pub fn automation_pause(ctx: Context<AutomationPause>) -> Result<()> {
        automation_pause::handler(ctx)
    }

    /// Resumes a paused automation.
    pub fn automation_resume(ctx: Context<AutomationResume>) -> Result<()> {
        automation_resume::handler(ctx)
    }

    /// Resets an automation's next instruction.
    pub fn automation_reset(ctx: Context<AutomationReset>) -> Result<()> {
        automation_reset::handler(ctx)
    }

    /// Allows an owner to update the mutable properties of an automation.
    pub fn automation_update(
        ctx: Context<AutomationUpdate>,
        settings: AutomationSettings,
    ) -> Result<()> {
        automation_update::handler(ctx, settings)
    }

    /// Allows an owner to withdraw from an automation's lamport balance.
    pub fn automation_withdraw(ctx: Context<AutomationWithdraw>, amount: u64) -> Result<()> {
        automation_withdraw::handler(ctx, amount)
    }
}
