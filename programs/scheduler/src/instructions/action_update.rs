use {
    crate::state::*,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(ixs: Vec<InstructionData>)]
pub struct ActionUpdate<'info> {
    #[account(
        mut,
        seeds = [
            SEED_ACTION, 
            action.task.as_ref(),
            action.id.to_be_bytes().as_ref(),
        ],
        bump,
    )]
    pub action: Account<'info, Action>,
    
    #[account()]
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

pub fn handler(
    ctx: Context<ActionUpdate>,
    ixs: Vec<InstructionData>,
) -> Result<()> {
    let action = &mut ctx.accounts.action;

    // TODO verify ixs
    action.ixs = ixs;

    Ok(())
}