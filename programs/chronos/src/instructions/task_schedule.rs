use {
    crate::{state::*, errors::*},
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(
    instruction_data: InstructionData, 
    execute_at: u64, 
    bump: u8
)]
pub struct TaskSchedule<'info> {
    #[account(
        seeds = [SEED_DAEMON, daemon.owner.as_ref()],
        bump = daemon.bump,
        has_one = owner,
        owner = crate::ID
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        init,
        seeds = [SEED_TASK, daemon.key().as_ref()],
        bump = bump,
        payer = owner,
        space = 8 + size_of::<Task>() + std::mem::size_of_val(&instruction_data),
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<TaskSchedule>,
    instruction_data: InstructionData,
    execute_at: u64,
    bump: u8,
) -> ProgramResult {
    let daemon = &ctx.accounts.daemon;
    let task = &mut ctx.accounts.task;

    // Validate the daemon is the only required signer on the instruction.
    // If the instruction has other required signers, we should just fail now.
    for acc in instruction_data.keys.as_slice() {
        require!(
            !acc.is_signer || acc.pubkey == daemon.key(), 
            ErrorCode::InvalidSignatory
        );
    }

    task.daemon = daemon.key();
    task.instruction_data = instruction_data;
    task.is_executed = false;
    task.execute_at = execute_at;
    task.bump = bump;

    return Ok(());
}
