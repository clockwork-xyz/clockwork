use {
    crate::{state::*, errors::CronosError},
    anchor_lang::prelude::*,
    solana_program::sysvar
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
            SEED_DAEMON, 
            daemon.owner.as_ref()
        ],
        bump = daemon.bump,
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        mut,
        seeds = [
            SEED_FEE,
            fee.daemon.as_ref()
        ],
        bump = fee.bump,
        constraint = fee.daemon == daemon.key(),
    )]
    pub fee: Account<'info, Fee>,

    #[account(
        mut,
        seeds = [
            SEED_TASK, 
            task.daemon.as_ref(),
            task.id.to_be_bytes().as_ref(),
        ],
        bump = task.bump,
        has_one = daemon,
        constraint = task.exec_at.is_some() && task.exec_at <= Some(clock.unix_timestamp) @ CronosError::TaskNotDue,
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(ctx: Context<TaskExec>) -> Result<()> {
    let bot = &mut ctx.accounts.bot;
    let config = &ctx.accounts.config;
    let daemon = &mut ctx.accounts.daemon;
    let fee = &mut ctx.accounts.fee;
    let task = &mut ctx.accounts.task;

    task.exec(&ctx.remaining_accounts.iter().as_slice(), bot, config, daemon, fee)
}
