use {
    crate::state::*,
    anchor_lang::prelude::*,
};


#[derive(Accounts)]
pub struct TaskClose<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub close_to: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.authority.as_ref(),
            queue.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = authority,
        constraint = queue.status == QueueStatus::Pending || queue.status == QueueStatus::Paused
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
        constraint = task.id == queue.task_count.checked_sub(1).unwrap(),
        close = close_to
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(ctx: Context<TaskClose>) -> Result<()> {
    let queue = &mut ctx.accounts.queue;
    queue.task_count = queue.task_count.checked_sub(1).unwrap();
    Ok(())
}
