use {
    crate::state::*,
    anchor_lang::prelude::*,
    solana_program::{system_program, sysvar},
    std::mem::size_of
};

#[derive(Accounts)]
#[instruction(
    ixs: Vec<InstructionData>,
    schedule: String,
    bump: u8
)]
pub struct AdminTaskOpen<'info> {
    #[account(mut, address = config.admin)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [SEED_AUTHORITY], 
        bump = authority.bump, 
    )]
    pub authority: Account<'info, Authority>,
    
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_DAEMON, 
            daemon.owner.as_ref()
        ],
        bump = daemon.bump,
        constraint = daemon.owner == authority.key(),
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(
        init,
        seeds = [
            SEED_TASK, 
            daemon.key().as_ref(),
            daemon.task_count.to_be_bytes().as_ref(),
        ],
        bump,
        payer = admin,
        space = 8 + size_of::<Task>() + borsh::to_vec(&ixs).unwrap().len(),
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(
    ctx: Context<AdminTaskOpen>, 
    ixs: Vec<InstructionData>,
    schedule: String,
    bump: u8
) -> Result<()> {
    let clock = &ctx.accounts.clock;
    let daemon = &mut ctx.accounts.daemon;
    let task = &mut ctx.accounts.task;

    task.open(bump, clock, daemon, ixs, schedule)
}
