use {
    crate::{events::*, state::*, errors::CronosError},
    anchor_lang::{prelude::*, solana_program::sysvar},
};

#[derive(Accounts)]
#[instruction()]
pub struct TaskExec<'info> {
    #[account(mut)]
    pub bot: Signer<'info>,

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
            SEED_FEE,
            fee.queue.as_ref()
        ],
        bump = fee.bump,
        constraint = fee.queue == queue.key(),
    )]
    pub fee: Account<'info, Fee>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE,
            queue.owner.as_ref()
        ],
        bump = queue.bump,
    )]
    pub queue: Account<'info, Queue>,

    #[account(
        mut,
        seeds = [
            SEED_TASK, 
            task.queue.as_ref(),
            task.id.to_be_bytes().as_ref(),
        ],
        bump = task.bump,
        has_one = queue,
        constraint = task.exec_at.is_some() && task.exec_at <= Some(clock.unix_timestamp) @ CronosError::TaskNotDue,
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(ctx: Context<TaskExec>) -> Result<()> {
    let bot = &mut ctx.accounts.bot;
    let clock = &ctx.accounts.clock;
    let config = &ctx.accounts.config;
    let fee = &mut ctx.accounts.fee;
    let queue = &mut ctx.accounts.queue;
    let task = &mut ctx.accounts.task;

    task.exec(&ctx.remaining_accounts.iter().as_slice(), bot, config, fee, queue)?;

    emit!(TaskExecuted {
        bot: bot.key(),
        task: task.key(),
        ts: clock.unix_timestamp,
    });

    Ok(())
}
