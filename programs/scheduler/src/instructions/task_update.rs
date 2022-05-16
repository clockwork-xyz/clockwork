use {
    crate::state::*,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(ixs: Vec<InstructionData>)]
pub struct TaskUpdate<'info> {
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
    
    #[account()]
    pub owner: Signer<'info>,

    #[account(
        seeds = [
            SEED_MANAGER, 
            manager.owner.as_ref()
        ],
        bump,
        has_one = owner,
    )]
    pub manager: Account<'info, Manager>,

    #[account(
        seeds = [
            SEED_QUEUE, 
            queue.manager.as_ref(),
            queue.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = manager,
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(
    ctx: Context<TaskUpdate>,
    ixs: Vec<InstructionData>,
) -> Result<()> {
    let task = &mut ctx.accounts.task;

    // TODO verify ixs
    task.ixs = ixs;

    Ok(())
}