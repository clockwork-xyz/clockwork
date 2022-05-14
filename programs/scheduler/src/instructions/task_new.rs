use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::{system_program, sysvar}},
    std::mem::size_of
};

#[derive(Accounts)]
#[instruction(schedule: String)]
pub struct TaskNew<'info> {
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account()]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
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
        init,
        seeds = [
            SEED_TASK, 
            queue.key().as_ref(),
            queue.task_count.to_be_bytes().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Task>(), // + borsh::to_vec(&ixs).unwrap().len(),
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(ctx: Context<TaskNew>, schedule: String) -> Result<()> {
    let clock = &ctx.accounts.clock;
    let queue = &mut ctx.accounts.queue;
    let task = &mut ctx.accounts.task;

    task.new(clock, queue, schedule)?;

    Ok(())
}
