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
            queue.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = authority,
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
    // TODO suport new schedule
    // TODO re-allocate aaccount space
    task.ixs = ixs;

    Ok(())
}
