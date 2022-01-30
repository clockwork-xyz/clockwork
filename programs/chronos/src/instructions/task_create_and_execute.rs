use solana_program::{instruction::Instruction, program::invoke_signed};

use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(instruction_data: InstructionData, bump: u8)]
pub struct TaskCreateAndExecute<'info> {
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [SEED_DAEMON, signer.key().as_ref()],
        bump = daemon.bump,
        constraint = daemon.owner == signer.key(),
        owner = crate::ID
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        init,
        seeds = [SEED_TASK, daemon.key().as_ref()],
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
    ctx: Context<TaskCreateAndExecute>,
    instruction_data: InstructionData,
    bump: u8,
) -> ProgramResult {
    // Get accounts.
    let clock = &ctx.accounts.clock;
    let daemon = &ctx.accounts.daemon;
    let task = &mut ctx.accounts.task;

    // Initialize task account.
    task.daemon = daemon.key();
    task.instruction_data = instruction_data;
    task.is_executed = true;
    task.execute_at = clock.unix_timestamp as u64;
    task.bump = bump;

    // Process the task.
    invoke_signed(
        &Instruction::from(&task.instruction_data),
        &mut ctx.remaining_accounts.iter().as_slice(),
        &[&[SEED_DAEMON, daemon.owner.key().as_ref(), &[daemon.bump]]],
    )?;

    return Ok(());
}
