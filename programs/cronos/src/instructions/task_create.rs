use {
    crate::{state::*, errors::*},
    anchor_lang::prelude::*,
    solana_program::{system_program, sysvar},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(
    ix: InstructionData,
    schedule: TaskSchedule,
    bump: u8,
)]
pub struct TaskCreate<'info> {
    #[account(
        address = sysvar::clock::ID,
        constraint = schedule.exec_at >= clock.unix_timestamp - 10 @ ErrorCode::InvalidExecAtStale
    )]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        owner = crate::ID,
    )]
    pub config: Account<'info, Config>,

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

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_TASK, 
            daemon.key().as_ref(),
            daemon.task_count.to_be_bytes().as_ref(),
        ],
        bump = bump,
        payer = owner,
        space = 8 + size_of::<Task>() + borsh::to_vec(&ix).unwrap().len(),
    )]
    pub task: Account<'info, Task>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<TaskCreate>,
    ix: InstructionData,
    schedule: TaskSchedule,
    bump: u8,
) -> ProgramResult {
    // Get accounts.
    let clock = &ctx.accounts.clock;
    let config = &ctx.accounts.config;
    let daemon = &mut ctx.accounts.daemon;
    let task = &mut ctx.accounts.task;

    clock.unix_timestamp.checked_add(10).unwrap();

    // Validate the task schedule.
    require!(schedule.exec_at <= schedule.stop_at, ErrorCode::InvalidChronology);
    require!(schedule.recurr >= 0, ErrorCode::InvalidRecurrNegative);
    require!(schedule.recurr == 0 || schedule.recurr >= config.min_recurr, ErrorCode::InvalidRecurrBelowMin);

    // Reject the instruction if it has other signers besides the daemon.
    for acc in ix.accounts.as_slice() {
        require!(
            !acc.is_signer || acc.pubkey == daemon.key(), 
            ErrorCode::InvalidSignatory
        );
    }

    // Initialize task account.
    task.daemon = daemon.key();
    task.int = daemon.task_count;
    task.ix = ix;
    task.schedule = schedule;
    task.status = TaskStatus::Queued;
    task.bump = bump;

    // Increment daemon task counter.
    daemon.task_count = daemon.task_count.checked_add(1).unwrap();

    return Ok(());
}
