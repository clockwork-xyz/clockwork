extern crate chrono;
extern crate clockwork_cron;

pub mod anchor;
pub mod errors;
pub mod id;
pub mod payer;
pub mod responses;
pub mod state;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

#[program]
pub mod clockwork_scheduler {
    use super::*;

    pub fn admin_config_update(
        ctx: Context<AdminConfigUpdate>,
        settings: ConfigSettings,
    ) -> Result<()> {
        admin_config_update::handler(ctx, settings)
    }

    pub fn admin_fee_claim(ctx: Context<AdminFeeClaim>, amount: u64) -> Result<()> {
        admin_fee_claim::handler(ctx, amount)
    }

    pub fn fee_claim(ctx: Context<FeeClaim>, amount: u64) -> Result<()> {
        fee_claim::handler(ctx, amount)
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn queue_close(ctx: Context<QueueClose>) -> Result<()> {
        queue_close::handler(ctx)
    }

    pub fn queue_deposit(ctx: Context<QueueDeposit>, amount: u64) -> Result<()> {
        queue_deposit::handler(ctx, amount)
    }

    pub fn queue_pause(ctx: Context<QueuePause>) -> Result<()> {
        queue_pause::handler(ctx)
    }

    pub fn queue_process(ctx: Context<QueueProcess>) -> Result<()> {
        queue_process::handler(ctx)
    }

    pub fn queue_new(
        ctx: Context<QueueNew>,
        balance: u64,
        name: String,
        schedule: String,
    ) -> Result<()> {
        queue_new::handler(ctx, balance, name, schedule)
    }

    pub fn queue_resume(ctx: Context<QueueResume>, skip_forward: bool) -> Result<()> {
        queue_resume::handler(ctx, skip_forward)
    }

    pub fn queue_update(ctx: Context<QueueUpdate>, schedule: String) -> Result<()> {
        queue_update::handler(ctx, schedule)
    }

    pub fn queue_withdraw(ctx: Context<QueueWithdraw>, amount: u64) -> Result<()> {
        queue_withdraw::handler(ctx, amount)
    }

    pub fn task_close(ctx: Context<TaskClose>) -> Result<()> {
        task_close::handler(ctx)
    }

    pub fn task_exec(ctx: Context<TaskExec>) -> Result<()> {
        task_exec::handler(ctx)
    }

    pub fn task_new(ctx: Context<TaskNew>, ixs: Vec<InstructionData>) -> Result<()> {
        task_new::handler(ctx, ixs)
    }

    pub fn task_update(ctx: Context<TaskUpdate>, ixs: Vec<InstructionData>) -> Result<()> {
        task_update::handler(ctx, ixs)
    }
}
