pub mod errors;
mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*, state::InstructionData};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod chronos {
    use super::*;

    pub fn daemon_create(ctx: Context<DaemonCreate>, bump: u8) -> ProgramResult {
        daemon_create::handler(ctx, bump)
    }

    pub fn daemon_invoke(
        ctx: Context<DaemonInvoke>,
        instruction_data: InstructionData,
    ) -> ProgramResult {
        daemon_invoke::handler(ctx, instruction_data)
    }

    pub fn task_execute(ctx: Context<TaskProcess>) -> ProgramResult {
        task_execute::handler(ctx)
    }

    pub fn task_schedule(
        ctx: Context<TaskSchedule>,
        instruction_data: InstructionData,
        execute_at: u64,
        bump: u8,
    ) -> ProgramResult {
        task_schedule::handler(ctx, instruction_data, execute_at, bump)
    }
}
