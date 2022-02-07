use {
    crate::{errors::*, state::*},
    anchor_lang::prelude::*,
    solana_program::system_program,
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(
    bump: u8,
)]
pub struct TaskRepeat<'info> {
    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        owner = crate::ID
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_DAEMON, 
            daemon.owner.key().as_ref()
        ],
        bump = daemon.bump,
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
        payer = worker,
        space = 8 + size_of::<Task>() + std::mem::size_of_val(&prev_task.instruction_data),
    )]
    pub next_task: Account<'info, Task>,

    #[account(
        mut,
        seeds = [
            SEED_TASK, 
            prev_task.daemon.as_ref(),
            prev_task.id.to_be_bytes().as_ref(),
        ],
        bump = prev_task.bump,
        has_one = daemon,
        constraint = prev_task.status == TaskStatus::Repeatable @ ErrorCode::TaskNotRepeatable,
        owner = crate::ID
    )]
    pub prev_task: Account<'info, Task>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account()]
    pub worker: Signer<'info>,
}

pub fn handler(
    ctx: Context<TaskRepeat>, 
    bump: u8, 
) -> ProgramResult {
    // Get accounts.
    let config = &ctx.accounts.config;
    let daemon = &mut ctx.accounts.daemon;
    let next_task = &mut ctx.accounts.next_task;
    let prev_task = &mut ctx.accounts.prev_task;
    let worker = &ctx.accounts.worker;

    // Initialize next_task account
    next_task.daemon = prev_task.daemon;
    next_task.id = daemon.task_count;
    next_task.instruction_data = prev_task.instruction_data.clone();
    next_task.status = TaskStatus::Pending;
    next_task.execute_at = prev_task.execute_at.checked_add(prev_task.repeat_every).unwrap();
    next_task.repeat_every = prev_task.repeat_every;
    next_task.repeat_until = prev_task.repeat_until;
    next_task.bump = bump;
    
    // Mark previous task as executed.
    prev_task.status = TaskStatus::Executed;

    // Increment daemon total task count.
    daemon.task_count = daemon.task_count.checked_add(1).unwrap();

    // Transfer lamports from daemon to worker.
    **daemon.to_account_info().try_borrow_mut_lamports()? -= config.worker_fee;
    **worker.to_account_info().try_borrow_mut_lamports()? += config.worker_fee;

    Ok(())
}
