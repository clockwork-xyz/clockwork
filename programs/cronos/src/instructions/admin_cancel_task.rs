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
        seeds = [SEED_AUTHORITY], 
        bump = authority.bump, 
        owner = crate::ID
    )]
    pub authority: Account<'info, Authority>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        owner = crate::ID,
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [
            SEED_DAEMON, 
            daemon.owner.as_ref()
        ],
        bump = daemon.bump,
        constraint = daemon.owner == authority.key(),
        owner = crate::ID,
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
        constraint = task.daemon == daemon.key(),
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
