use {
    crate::state::*,
    anchor_lang::prelude::*,
};


#[derive(Accounts)]
#[instruction(ixs: Vec<InstructionData>)]
pub struct TaskUpdate<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(
        seeds = [
            SEED_QUEUE, 
            queue.authority.as_ref(),
            queue.name.as_bytes(),
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
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(
    ctx: Context<TaskUpdate>,
    ixs: Vec<InstructionData>,
) -> Result<()> {
    let task = &mut ctx.accounts.task;

    // TODO verify ixs
    // TODO re-allocate aaccount space
    task.ixs = ixs;

    Ok(())
}
