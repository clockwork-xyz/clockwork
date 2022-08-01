use {
    crate::{errors::ClockworkError, instructions::utils::is_spam, state::*},
    anchor_lang::{prelude::*, system_program},
    clockwork_pool::state::Pool,
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct TaskExec<'info> {
    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init_if_needed,
        seeds = [
            SEED_FEE,
            worker.key().as_ref()
        ],
        bump,
        payer = worker,
        space = 8 + size_of::<Fee>(),
    )]
    pub fee: Account<'info, Fee>,

    #[account()]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.authority.as_ref(),
            queue.name.as_bytes(),
        ],
        bump,
        constraint = match queue.status {
            QueueStatus::Processing { task_id } => task_id == task.id,
            _ => false,
        } @ ClockworkError::InvalidQueueStatus
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
    // Load accounts
    let task = &mut ctx.accounts.task;
    let config = &ctx.accounts.config;
    let fee = &mut ctx.accounts.fee;
    let pool = &ctx.accounts.pool;
    let queue = &mut ctx.accounts.queue;
    let worker = &mut ctx.accounts.worker;

    // Validate the worker is authorized to execute this task
    if is_spam(&config, pool, queue, worker)? {
        fee.pay_to_admin(config.spam_penalty, queue)?;
        return Ok(());
    }

    // Execute the task
    let account_infos = &mut ctx.remaining_accounts.clone().to_vec();
    let queue_bump = *ctx.bumps.get("queue").unwrap();
    task.exec(account_infos, config, fee, queue, queue_bump, worker)
}
