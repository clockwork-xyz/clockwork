use crate::errors::ClockworkError;

use {
    crate::state::*,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct QueueCrank<'info> {    
    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.authority.as_ref(),
            queue.name.as_bytes(),
        ],
        bump,
        constraint = queue.next_instruction.is_some()
    )]
    pub queue: Account<'info, Queue>,

    #[account(mut)]
    pub worker: Signer<'info>,
}

pub fn handler(ctx: Context<QueueCrank>) -> Result<()> {
    // Get accounts
    let queue = &mut ctx.accounts.queue;
    let worker = &ctx.accounts.worker;

    // Crank the queue
    match &queue.clone().next_instruction {
        None => { return Err(ClockworkError::NoInstruction.into())}
        Some(next_instruction) => {
            let bump = ctx.bumps.get("queue").unwrap();
            queue.crank(ctx.remaining_accounts, *bump, next_instruction, worker)?;
        }
    }
    
    // TODO Pay fees to worker
    // TODO Dynamically resize queue account, if needed

    Ok(())
}
