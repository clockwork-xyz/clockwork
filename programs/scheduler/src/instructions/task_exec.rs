use {
    crate::{errors::CronosError, instructions::utils::is_spam, state::*},
    anchor_lang::{prelude::*, solana_program::sysvar, system_program},
    cronos_pool::state::Pool
};

#[derive(Accounts)]
pub struct TaskExec<'info> {
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(
        mut,
        seeds = [
            SEED_FEE,
            queue.key().as_ref()
        ],
        bump,
        has_one = queue
    )]
    pub fee: Account<'info, Fee>,

    #[account(seeds = [SEED_MANAGER, manager.authority.as_ref()], bump)]
    pub manager: Account<'info, Manager>,

    #[account()]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.manager.as_ref(),
            queue.id.to_be_bytes().as_ref(),
        ],
        bump,
        constraint = match queue.status {
            QueueStatus::Processing { task_id } => task_id == task.id,
            _ => false,
        } @ CronosError::InvalidQueueStatus
    )]
    pub queue: Account<'info, Queue>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [
            SEED_TASK,
            task.queue.as_ref(),
            task.id.to_be_bytes().as_ref()
        ],
        bump,
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub worker: Signer<'info>,
}

pub fn handler(ctx: Context<TaskExec>) -> Result<()> {
    let task = &mut ctx.accounts.task;
    let clock = &ctx.accounts.clock;
    let config = &ctx.accounts.config;
    let fee = &mut ctx.accounts.fee;
    let manager = &ctx.accounts.manager;
    let pool = &ctx.accounts.pool;
    let queue = &mut ctx.accounts.queue;
    let system_program = &ctx.accounts.system_program;
    let worker = &mut ctx.accounts.worker;

    // Validate the worker is authorized to execute this task
    if is_spam(clock, &config, fee, pool, queue, system_program, worker)? {
        return Ok(());
    }

    let account_infos = &mut ctx.remaining_accounts.clone().to_vec();

    let manager_bump = *ctx.bumps.get("manager").unwrap();
    task.exec(account_infos, config, fee, manager, manager_bump, queue, worker)?;
    Ok(())
}
