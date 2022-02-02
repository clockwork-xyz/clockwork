pub mod errors;
mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*, state::*};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod chronos {
    use super::*;

    pub fn config_update_admin_authority(
        ctx: Context<ConfigUpdateAdminAuthority>,
        new_admin_authority: Pubkey,
    ) -> ProgramResult {
        config_update_admin_authority::handler(ctx, new_admin_authority)
    }

    pub fn config_update_frame_interval(
        ctx: Context<ConfigUpdateFrameInterval>,
        new_frame_interval: u64,
    ) -> ProgramResult {
        config_update_frame_interval::handler(ctx, new_frame_interval)
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

    pub fn daemon_create(ctx: Context<DaemonCreate>, bump: u8) -> ProgramResult {
        daemon_create::handler(ctx, bump)
    }

    pub fn daemon_invoke(
        ctx: Context<DaemonInvoke>,
        instruction_data: InstructionData,
    ) -> ProgramResult {
        daemon_invoke::handler(ctx, instruction_data)
    }

    pub fn frame_create(
        ctx: Context<WindowCreate>,
        timestamp: u64,
        frame_bump: u8,
        list_bump: u8,
    ) -> ProgramResult {
        frame_create::handler(ctx, timestamp, frame_bump, list_bump)
    }

    pub fn initialize(
        ctx: Context<Initialize>,
        authority_bump: u8,
        config_bump: u8,
        treasury_bump: u8,
    ) -> ProgramResult {
        initialize::handler(ctx, authority_bump, config_bump, treasury_bump)
    }

    pub fn revenue_collect(ctx: Context<RevenueCollect>) -> ProgramResult {
        revenue_collect::handler(ctx)
    }

    pub fn revenue_create(ctx: Context<RevenueCreate>, bump: u8) -> ProgramResult {
        revenue_create::handler(ctx, bump)
    }

    pub fn task_create(
        ctx: Context<TaskCreate>,
        instruction_data: InstructionData,
        execute_at: u64,
        repeat_every: u64,
        repeat_until: u64,
        task_bump: u8,
        task_element_bump: u8,
    ) -> ProgramResult {
        task_create::handler(
            ctx,
            instruction_data,
            execute_at,
            repeat_every,
            repeat_until,
            task_bump,
            task_element_bump,
        )
    }

    pub fn task_execute(ctx: Context<TaskProcess>) -> ProgramResult {
        task_execute::handler(ctx)
    }

    pub fn task_repeat(
        ctx: Context<TaskRepeat>,
        next_task_bump: u8,
        next_task_element_bump: u8,
    ) -> ProgramResult {
        task_repeat::handler(ctx, next_task_bump, next_task_element_bump)
    }
}
