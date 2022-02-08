pub mod errors;
mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*, state::*};

declare_id!("GYJGrWYmH9L3JtrHGPVnMuUtLtdLkqvNVBV93oGtCtDs");

#[program]
pub mod cronos {
    use super::*;

    pub fn admin_schedule_health_check(
        ctx: Context<AdminScheduleHealthCheck>,
        bump: u8,
    ) -> ProgramResult {
        admin_schedule_health_check::handler(ctx, bump)
    }

    pub fn config_update_admin(
        ctx: Context<ConfigUpdateAdmin>,
        new_admin: Pubkey,
    ) -> ProgramResult {
        config_update_admin::handler(ctx, new_admin)
    }

    pub fn config_update_program_fee(
        ctx: Context<ConfigUpdateProgramFee>,
        new_program_fee: u64,
    ) -> ProgramResult {
        config_update_program_fee::handler(ctx, new_program_fee)
    }

    pub fn config_update_worker_fee(
        ctx: Context<ConfigUpdateWorkerFee>,
        new_worker_fee: u64,
    ) -> ProgramResult {
        config_update_worker_fee::handler(ctx, new_worker_fee)
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

    pub fn fee_collect(ctx: Context<FeeCollect>) -> ProgramResult {
        fee_collect::handler(ctx)
    }

    pub fn initialize(
        ctx: Context<Initialize>,
        authority_bump: u8,
        config_bump: u8,
        daemon_bump: u8,
        fee_bump: u8,
        health_bump: u8,
        treasury_bump: u8,
    ) -> ProgramResult {
        initialize::handler(
            ctx,
            authority_bump,
            config_bump,
            daemon_bump,
            fee_bump,
            health_bump,
            treasury_bump,
        )
    }

    pub fn health_check(ctx: Context<HealthCheck>) -> ProgramResult {
        health_check::handler(ctx)
    }

    pub fn task_cancel(ctx: Context<TaskCancel>) -> ProgramResult {
        task_cancel::handler(ctx)
    }

    pub fn task_create(
        ctx: Context<TaskCreate>,
        instruction_data: InstructionData,
        execute_at: u64,
        repeat_every: u64,
        repeat_until: u64,
        bump: u8,
    ) -> ProgramResult {
        task_create::handler(
            ctx,
            instruction_data,
            execute_at,
            repeat_every,
            repeat_until,
            bump,
        )
    }

    pub fn task_execute(ctx: Context<TaskProcess>) -> ProgramResult {
        task_execute::handler(ctx)
    }
}
