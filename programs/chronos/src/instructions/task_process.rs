use {
    crate::state::*,
    anchor_lang::prelude::*,
    solana_program::{instruction::Instruction, program::invoke_signed},
};

#[derive(Accounts)]
#[instruction()]
pub struct TaskProcess<'info> {
    #[account(
        seeds = [SEED_AGENT, agent.owner.key().as_ref()],
        bump = agent.bump,
        owner = crate::ID
    )]
    pub agent: Account<'info, Agent>,

    #[account(
        mut,
        seeds = [SEED_TASK, agent.key().as_ref()],
        bump = task.bump,
        constraint = task.is_processed == false,
        owner = crate::ID
    )]
    pub task: Account<'info, Task>,

    #[account()]
    pub worker: Signer<'info>,
}

pub fn handler(ctx: Context<TaskProcess>) -> ProgramResult {
    // Get accounts.
    let agent = &ctx.accounts.agent;
    let task = &mut ctx.accounts.task;

    invoke_signed(
        &Instruction::from(&task.instruction_data),
        &mut ctx.remaining_accounts.iter().as_slice(),
        &[&[SEED_AGENT, agent.owner.key().as_ref(), &[agent.bump]]],
    )?;

    task.is_processed = true;

    return Ok(());
}
