use crate::state::*;

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction()]
pub struct AdminCancelTask<'info> {
    #[account(
        mut, 
        address = config.admin
    )]
    pub admin: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_TASK, 
            task.daemon.as_ref(),
            task.int.to_be_bytes().as_ref(),
        ],
        bump = task.bump,
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(ctx: Context<AdminCancelTask>) -> Result<()> {
    let task = &mut ctx.accounts.task;
    
    task.cancel()
}
