use {
    crate::state::*,
    anchor_lang::prelude::*
};

#[derive(Accounts)]
#[instruction()]
pub struct TaskCancel<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        seeds = [
            SEED_QUEUE,
            queue.owner.as_ref()
        ],
        bump,
        has_one = owner,
    )]
    pub queue: Account<'info, Queue>,

    #[account(
        mut,
        seeds = [
            SEED_TASK, 
            task.queue.as_ref(),
            task.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = queue,
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(ctx: Context<TaskCancel>) -> Result<()> {
    let task = &mut ctx.accounts.task;
    let owner = &mut ctx.accounts.owner;
    
    task.cancel(owner)
}
