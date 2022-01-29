pub mod errors;
mod instructions;
pub mod state;

use {anchor_lang::prelude::*, instructions::*, state::InstructionData};
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod chronos {
    use super::*;

    pub fn agent_create(ctx: Context<AgentCreate>, bump: u8) -> ProgramResult {
        agent_create::handler(ctx, bump)
    }

    pub fn task_create(
        ctx: Context<TaskCreate>,
        instruction_data: InstructionData,
        bump: u8,
    ) -> ProgramResult {
        task_create::handler(ctx, instruction_data, bump)
    }

    pub fn task_process(ctx: Context<TaskProcess>) -> ProgramResult {
        task_process::handler(ctx)
    }
}
