use {
    crate::{errors::*, state::*},
    anchor_lang::prelude::*,
    solana_program::{instruction::Instruction, program::invoke_signed},
};

#[derive(Accounts)]
#[instruction()]
pub struct TaskProcess<'info> {
    pub clock: Sysvar<'info, Clock>,

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
        mut,
        seeds = [
            SEED_TASK, 
            task.daemon.as_ref(),
            task.id.to_be_bytes().as_ref(),
        ],
        bump = task.bump,
        has_one = daemon,
        constraint = task.status == TaskStatus::Pending @ ErrorCode::TaskNotPending,
        constraint = task.execute_at <= clock.unix_timestamp as u64 @ ErrorCode::TaskNotDue,
        owner = crate::ID
    )]
    pub task: Account<'info, Task>,

    #[account()]
    pub worker: Signer<'info>,
}

pub fn handler(ctx: Context<TaskProcess>) -> ProgramResult {
    let daemon = &mut ctx.accounts.daemon;
    let task = &mut ctx.accounts.task;

    let next_execution_frame: u64 = task.execute_at.checked_add(task.repeat_every).unwrap();
    if task.repeat_every > 0 && next_execution_frame <= task.repeat_until {
        task.status = TaskStatus::Repeat;
    } else {
        task.status = TaskStatus::Done;
    }

    daemon.executed_task_count = daemon.executed_task_count.checked_add(1).unwrap();

    invoke_signed(
        &Instruction::from(&task.instruction_data),
        &ctx.remaining_accounts.iter().as_slice(),
        &[&[SEED_DAEMON, daemon.owner.key().as_ref(), &[daemon.bump]]],
    )?;

    // TODO pay out bounty to worker

    Ok(())
}
