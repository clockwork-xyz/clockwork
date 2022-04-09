use {
    crate::state::*,
    anchor_lang::prelude::*
};

#[derive(Accounts)]
#[instruction()]
pub struct TaskCancel<'info> {
    #[account(
        seeds = [
            SEED_DAEMON, 
            daemon.owner.as_ref()
        ],
        bump = daemon.bump,
        has_one = owner,
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_TASK, 
            task.daemon.as_ref(),
            task.id.to_be_bytes().as_ref(),
        ],
        bump = task.bump,
        has_one = daemon,
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(ctx: Context<TaskCancel>) -> Result<()> {
    let task = &mut ctx.accounts.task;
    let owner = &mut ctx.accounts.owner;
    
    task.cancel(owner)
}
