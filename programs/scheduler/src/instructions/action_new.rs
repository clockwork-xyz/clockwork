use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of
};

#[derive(Accounts)]
#[instruction(
    bump: u8,
    ixs: Vec<InstructionData>,
)]
pub struct ActionNew<'info> {
    #[account(
        init,
        seeds = [
            SEED_ACTION, 
            task.key().as_ref(),
            task.action_count.to_be_bytes().as_ref(),
        ],
        bump,
        payer = owner,
        space = 8 + size_of::<Action>() + borsh::to_vec(&ixs).unwrap().len(),
    )]
    pub action: Account<'info, Action>,
    
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        seeds = [
            SEED_QUEUE, 
            queue.owner.as_ref()
        ],
        bump = queue.bump,
        has_one = owner,
    )]
    pub queue: Account<'info, Queue>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [
            SEED_TASK, 
            queue.key().as_ref(),
            task.id.to_be_bytes().as_ref(),
        ],
        bump = task.bump,
        has_one = queue,
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(
    ctx: Context<ActionNew>,
    bump: u8,
    ixs: Vec<InstructionData>,
) -> Result<()> {
    let action = &mut ctx.accounts.action;
    let task = &mut ctx.accounts.task;

    action.new(bump, ixs, task)
}
