pub mod errors;
pub mod state;

mod instructions;
mod pda;

use {anchor_lang::prelude::*, instructions::*, state::*};

declare_id!("7fNcRaPYHSBbqZuM5E87s3k6hX9DfPWCwiNbmLib2XvZ");

#[program]
pub mod cronos {
    use super::*;

    pub fn admin_cancel_task(ctx: Context<AdminCancelTask>) -> ProgramResult {
        admin_cancel_task::handler(ctx)
    }

    pub fn admin_create_task(
        ctx: Context<AdminCreateTask>,
        ix: InstructionData,
        exec_at: i64,
        stop_at: i64,
        recurr: i64,
        bump: u8,
    ) -> ProgramResult {
        admin_create_task::handler(ctx, ix, exec_at, stop_at, recurr, bump)
    }

    pub fn admin_initialize(
        ctx: Context<AdminInitialize>,
        authority_bump: u8,
        config_bump: u8,
        daemon_bump: u8,
        fee_bump: u8,
        health_bump: u8,
        treasury_bump: u8,
    ) -> ProgramResult {
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

    pub fn admin_reset_health(ctx: Context<AdminResetHealth>) -> ProgramResult {
        admin_reset_health::handler(ctx)
    }

    pub fn admin_update_admin(ctx: Context<AdminUpdateAdmin>, new_admin: Pubkey) -> ProgramResult {
        admin_update_admin::handler(ctx, new_admin)
    }

    pub fn admin_update_min_recurr(
        ctx: Context<AdminUpdateMinRecurr>,
        new_min_recurr: i64,
    ) -> ProgramResult {
        admin_update_min_recurr::handler(ctx, new_min_recurr)
    }

    pub fn admin_update_program_fee(
        ctx: Context<AdminUpdateProgramFee>,
        new_program_fee: u64,
    ) -> ProgramResult {
        admin_update_program_fee::handler(ctx, new_program_fee)
    }

    pub fn admin_update_worker_fee(
        ctx: Context<AdminUpdateWorkerFee>,
        new_worker_fee: u64,
    ) -> ProgramResult {
        admin_update_worker_fee::handler(ctx, new_worker_fee)
    }

    pub fn daemon_create(
        ctx: Context<DaemonCreate>,
        daemon_bump: u8,
        fee_bump: u8,
    ) -> ProgramResult {
        daemon_create::handler(ctx, daemon_bump, fee_bump)
    }

    pub fn daemon_invoke(
        ctx: Context<DaemonInvoke>,
        instruction_data: InstructionData,
    ) -> ProgramResult {
        daemon_invoke::handler(ctx, instruction_data)
    }

    pub fn daemon_widthdraw(ctx: Context<DaemonWidthdraw>, amount: u64) -> ProgramResult {
        daemon_widthdraw::handler(ctx, amount)
    }

    pub fn fee_collect(ctx: Context<FeeCollect>) -> ProgramResult {
        fee_collect::handler(ctx)
    }

    pub fn health_check(ctx: Context<HealthCheck>) -> ProgramResult {
        health_check::handler(ctx)
    }

    pub fn task_cancel(ctx: Context<TaskCancel>) -> ProgramResult {
        task_cancel::handler(ctx)
    }

    pub fn task_create(
        ctx: Context<TaskCreate>,
        ix: InstructionData,
        exec_at: i64,
        stop_at: i64,
        recurr: i64,
        bump: u8,
    ) -> ProgramResult {
        task_create::handler(ctx, ix, exec_at, stop_at, recurr, bump)
    }

    pub fn task_execute(ctx: Context<TaskExecute>) -> ProgramResult {
        task_execute::handler(ctx)
    }
}
