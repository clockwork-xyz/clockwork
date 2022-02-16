use {
    crate::state::*,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction()]
pub struct AdminCancelTask<'info> {
    #[account(mut, address = config.admin)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        owner = crate::ID,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_TASK, 
            task.daemon.as_ref(),
            task.int.to_be_bytes().as_ref(),
        ],
        bump = task.bump,
        owner = crate::ID
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(ctx: Context<AdminCancelTask>) -> ProgramResult {
    // Get accounts.
    let task = &mut ctx.accounts.task;

    // Mark task as cancelled.
    task.status = TaskStatus::Cancelled;

    Ok(())
}
