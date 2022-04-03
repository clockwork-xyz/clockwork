use {
    crate::state::*,
    anchor_lang::prelude::*,
    solana_program::{system_program, sysvar},
    std::mem::size_of
};

#[derive(Accounts)]
#[instruction(
    ix: InstructionData,
    schedule: String,
    bump: u8,
)]
pub struct TaskOpen<'info> {
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        mut,
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
        init,
        seeds = [
            SEED_TASK, 
            daemon.key().as_ref(),
            daemon.task_count.to_be_bytes().as_ref(),
        ],
        bump,
        payer = owner,
        space = 8 + size_of::<Task>() + borsh::to_vec(&ix).unwrap().len(),
    )]
    pub task: Account<'info, Task>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<TaskOpen>,
    ixs: Vec<InstructionData>,
    schedule: String,
    bump: u8,
) -> Result<()> {
    let clock = &ctx.accounts.clock;
    let daemon = &mut ctx.accounts.daemon;
    let task = &mut ctx.accounts.task;

    task.open(bump, clock, daemon, ixs, schedule)
}
