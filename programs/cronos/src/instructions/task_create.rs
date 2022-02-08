use {
    crate::{state::*, errors::*},
    anchor_lang::prelude::*,
    solana_program::{system_program, sysvar},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(
    instruction_data: InstructionData,
    exec_at: i64, 
    stop_at: i64,
    recurr: i64,
    bump: u8,
)]
pub struct TaskCreate<'info> {
    #[account(
        address = sysvar::clock::ID,
        constraint = exec_at >= clock.unix_timestamp @ ErrorCode::InvalidExecAtStale
    )]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        mut,
        seeds = [
            SEED_DAEMON, 
            daemon.owner.as_ref()
        ],
        bump = daemon.bump,
        has_one = owner,
        owner = crate::ID
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        init,
        seeds = [
            SEED_TASK, 
            daemon.key().as_ref(),
            daemon.task_count.to_be_bytes().as_ref(),
        ],
        bump = bump,
        payer = owner,
        space = 8 + size_of::<Task>() + std::mem::size_of_val(&instruction_data),
        // TODO dont let tasks be scheduled in the past.
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<TaskCreate>,
    instruction_data: InstructionData,
    exec_at: i64, 
    stop_at: i64,
    recurr: i64,
    bump: u8,
) -> ProgramResult {
    // Get accounts.
    let daemon = &mut ctx.accounts.daemon;
    let task = &mut ctx.accounts.task;

    // Validate the scheduling chronology.
    require!(exec_at <= stop_at, ErrorCode::InvalidChronology);

    // Validate recurrence interval is not negative.
    require!(recurr >= 0, ErrorCode::InvalidRecurrNegative);

    // Validate the daemon is the only required signer on the instruction.
    // If the instruction has other required signers, we should just fail now.
    for acc in instruction_data.keys.as_slice() {
        require!(
            !acc.is_signer || acc.pubkey == daemon.key(), 
            ErrorCode::InvalidSignatory
        );
    }

    // Initialize task account.
    task.daemon = daemon.key();
    task.id = daemon.task_count;
    task.instruction_data = instruction_data;
    task.status = TaskStatus::Pending;
    task.exec_at = exec_at;
    task.stop_at = stop_at;
    task.recurr = recurr;
    task.bump = bump;

    // Increment daemon task counter.
    daemon.task_count = daemon.task_count.checked_add(1).unwrap();

    return Ok(());
}
