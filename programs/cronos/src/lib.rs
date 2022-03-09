extern crate chrono;
extern crate cronos_cron;

pub mod errors;
pub mod pda;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use instructions::*;
use state::*;

declare_id!("CronpZj5NbHj2Nb6WwEtf6A9anty9JfEQ1RnGoshQBaW");

#[program]
pub mod cronos {
    use super::*;

    pub fn admin_cancel_task(ctx: Context<AdminCancelTask>) -> Result<()> {
        admin_cancel_task::handler(ctx)
    }

    pub fn admin_create_task(
        ctx: Context<AdminCreateTask>,
        ix: InstructionData,
        schedule: String,
        bump: u8,
    ) -> Result<()> {
        admin_create_task::handler(ctx, ix, schedule, bump)
    }

    pub fn admin_initialize(
        ctx: Context<AdminInitialize>,
        authority_bump: u8,
        config_bump: u8,
        daemon_bump: u8,
        fee_bump: u8,
        health_bump: u8,
        treasury_bump: u8,
    ) -> Result<()> {
        admin_initialize::handler(
            ctx,
            authority_bump,
            config_bump,
            daemon_bump,
            fee_bump,
            health_bump,
            treasury_bump,
        )
    }

    pub fn admin_reset_health(ctx: Context<AdminResetHealth>) -> Result<()> {
        admin_reset_health::handler(ctx)
    }

    pub fn admin_update_admin(ctx: Context<AdminUpdateAdmin>, new_admin: Pubkey) -> Result<()> {
        admin_update_admin::handler(ctx, new_admin)
    }

    pub fn admin_update_min_recurr(
        ctx: Context<AdminUpdateMinRecurr>,
        new_min_recurr: i64,
    ) -> Result<()> {
        admin_update_min_recurr::handler(ctx, new_min_recurr)
    }

    pub fn admin_update_program_fee(
        ctx: Context<AdminUpdateProgramFee>,
        new_program_fee: u64,
    ) -> Result<()> {
        admin_update_program_fee::handler(ctx, new_program_fee)
    }

    pub fn admin_update_worker_fee(
        ctx: Context<AdminUpdateWorkerFee>,
        new_worker_fee: u64,
    ) -> Result<()> {
        admin_update_worker_fee::handler(ctx, new_worker_fee)
    }

    pub fn daemon_create(ctx: Context<DaemonCreate>, daemon_bump: u8, fee_bump: u8) -> Result<()> {
        daemon_create::handler(ctx, daemon_bump, fee_bump)
    }

    pub fn daemon_invoke(
        ctx: Context<DaemonInvoke>,
        instruction_data: InstructionData,
    ) -> Result<()> {
        daemon_invoke::handler(ctx, instruction_data)
    }

    pub fn daemon_widthdraw(ctx: Context<DaemonWidthdraw>, amount: u64) -> Result<()> {
        daemon_widthdraw::handler(ctx, amount)
    }

    pub fn fee_collect(ctx: Context<FeeCollect>) -> Result<()> {
        fee_collect::handler(ctx)
    }

    pub fn health_ping(ctx: Context<HealthPing>) -> Result<()> {
        health_ping::handler(ctx)
    }

    pub fn task_cancel(ctx: Context<TaskCancel>) -> Result<()> {
        task_cancel::handler(ctx)
    }

    pub fn task_create(
        ctx: Context<TaskCreate>,
        ix: InstructionData,
        schedule: String,
        bump: u8,
    ) -> Result<()> {
        task_create::handler(ctx, ix, schedule, bump)
    }

    pub fn task_execute(ctx: Context<TaskExecute>) -> Result<()> {
        task_execute::handler(ctx)
    }
}
