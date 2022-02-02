use {
    crate::{errors::*, state::*},
    anchor_lang::prelude::*,
    solana_program::{instruction::Instruction, program::invoke_signed},
};

#[derive(Accounts)]
#[instruction()]
pub struct TaskCancel<'info> {
    #[account(
        seeds = [
            SEED_DAEMON, 
            daemon.owner.key().as_ref()
        ],
        bump = daemon.bump,
        has_one = owner,
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
        owner = crate::ID
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn handler(ctx: Context<TaskCancel>) -> ProgramResult {
    // Get accounts.
    let task = &mut ctx.accounts.task;

    // Mark task as cancelled.
    task.status = TaskStatus::Cancelled;

    Ok(())
}
