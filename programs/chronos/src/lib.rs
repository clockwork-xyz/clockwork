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

    pub fn task_create(
        ctx: Context<TaskCreate>,
        instruction_data: InstructionData,
        execute_at: u64,
        bump: u8,
    ) -> ProgramResult {
        task_create::handler(ctx, instruction_data, execute_at, bump)
    }

    pub fn task_create_and_execute(
        ctx: Context<TaskCreateAndExecute>,
        instruction_data: InstructionData,
        bump: u8,
    ) -> ProgramResult {
        task_create_and_execute::handler(ctx, instruction_data, bump)
    }

    pub fn task_execute(ctx: Context<TaskProcess>) -> ProgramResult {
        task_execute::handler(ctx)
    }
}
