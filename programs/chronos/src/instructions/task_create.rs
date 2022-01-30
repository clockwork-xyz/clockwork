use crate::errors::ErrorCode;

use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(
    instruction_data: InstructionData, 
    execute_at: u64, 
    bump: u8
)]
pub struct TaskCreate<'info> {
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
        space = 8 + size_of::<Task>() + std::mem::size_of_val(&instruction_data),
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
    execute_at: u64,
    bump: u8,
) -> ProgramResult {
    // Get accounts.
    let daemon = &ctx.accounts.daemon;
    let task = &mut ctx.accounts.task;


    // Validate the daemon is the only required signer on the instruction.
    // If the instruction has other required signers, we should just fail now.
    for acc in task.instruction_data.keys.as_slice() {
        require!(
            !acc.is_signer || acc.pubkey == daemon.key(), 
            ErrorCode::InvalidSignatory
        );
    }

    // Initialize task account.
    task.daemon = daemon.key();
    task.instruction_data = instruction_data;
    task.is_executed = false;
    task.execute_at = execute_at;
    task.bump = bump;

    return Ok(());
}
