use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(instruction_data: InstructionData, bump: u8)]
pub struct TaskCreate<'info> {
    #[account(
        seeds = [SEED_AGENT, signer.key().as_ref()],
        bump = agent.bump,
        constraint = agent.owner == signer.key(),
        owner = crate::ID
    )]
    pub agent: Account<'info, Agent>,

    #[account(
        init,
        seeds = [SEED_TASK, agent.key().as_ref()],
        bump = bump,
        payer = signer,
        space = size_of::<Task>() + std::mem::size_of_val(&instruction_data),
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<TaskCreate>,
    instruction_data: InstructionData,
    bump: u8,
) -> ProgramResult {
    // Get accounts.
    let agent = &ctx.accounts.agent;
    let task = &mut ctx.accounts.task;

    // Initialize agent account.
    task.agent = agent.key();
    task.instruction_data = instruction_data;
    task.is_processed = false;
    task.bump = bump;

    return Ok(());
}
