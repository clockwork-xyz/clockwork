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
        seeds = [SEED_DAEMON, daemon.owner.key().as_ref()],
        bump = daemon.bump,
        owner = crate::ID
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        mut,
        seeds = [SEED_TASK, daemon.key().as_ref()],
        bump = task.bump,
        constraint = task.is_executed == false @ ErrorCode::TaskAlreadyExecuted,
        constraint = task.execute_at <= clock.unix_timestamp as u64 @ ErrorCode::TaskNotDue,
        owner = crate::ID
    )]
    pub task: Account<'info, Task>,

    #[account()]
    pub worker: Signer<'info>,
}

pub fn handler(ctx: Context<TaskProcess>) -> ProgramResult {
    let daemon = &ctx.accounts.daemon;
    let task = &mut ctx.accounts.task;

    invoke_signed(
        &Instruction::from(&task.instruction_data),
        &ctx.remaining_accounts.iter().as_slice(),
        &[&[SEED_DAEMON, daemon.owner.key().as_ref(), &[daemon.bump]]],
    )?;

    task.is_executed = true;

    return Ok(());
}
