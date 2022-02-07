use std::mem::size_of_val;

use solana_program::{log::sol_log};

use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program, solana_program::instruction::Instruction},
    std::mem::size_of,
};


#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct AdminScheduleHealthCheck<'info> {
    #[account(mut, address = config.admin)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [SEED_AUTHORITY], 
        bump = authority.bump, 
        owner = crate::ID
    )]
    pub authority: Account<'info, Authority>,
    
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        owner = crate::ID,
    )]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [
            SEED_DAEMON, 
            daemon.owner.as_ref()
        ],
        bump = bump,
        constraint = daemon.owner == authority.key(),
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        mut,
        seeds = [SEED_HEALTH],
        bump = health.bump,
        owner = crate::ID,
    )]
    pub health: Account<'info, Health>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(
        init,
        seeds = [
            SEED_TASK, 
            daemon.key().as_ref(),
            daemon.task_count.to_be_bytes().as_ref(),
        ],
        bump = bump,
        payer = admin,
        space = 8 + size_of::<Task>() + 100, // TODO optimize this
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(ctx: Context<AdminScheduleHealthCheck>, bump: u8) -> ProgramResult {
    // Get accounts.
    let authority = &ctx.accounts.authority;
    let clock = &ctx.accounts.clock;
    let daemon = &mut ctx.accounts.daemon;
    let health = &mut ctx.accounts.health;
    let task = &mut ctx.accounts.task;

    // Initialize daemon account.
    daemon.owner = authority.key();
    daemon.task_count = 0;
    daemon.bump = bump;

    // Setup the health account.
    let now = clock.unix_timestamp as u64;
    let execute_at = now.checked_add(1).unwrap();
    health.real_time = now;
    health.target_time = execute_at;


    // Create health check instruction
    let health_check_ix = InstructionData::from(
        Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new_readonly(clock.key(), false),
                AccountMeta::new_readonly(authority.key(), false),
                AccountMeta::new(daemon.key(), true),
                AccountMeta::new(health.key(), false),
            ],
            data: vec![],
        }
    );

    sol_log(format!("Health_check ix size: {:?}", size_of_val(&health_check_ix)).as_str());

    // Initialize task account.
    task.daemon = daemon.key();
    task.id = daemon.task_count;
    task.instruction_data = health_check_ix;
    task.status = TaskStatus::Pending;
    task.execute_at = execute_at;
    task.repeat_every = 1;
    task.repeat_until = u64::MAX;
    task.bump = bump;

    // Increment daemon task counter.
    daemon.task_count = daemon.task_count.checked_add(1).unwrap();

    Ok(())
}
