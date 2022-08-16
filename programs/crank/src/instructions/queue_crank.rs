use {
    crate::state::*,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct QueueCrank<'info> {
    #[account(
        mut,
        seeds = [
            SEED_EXEC,
            exec.queue.as_ref(),
            exec.id.to_be_bytes().as_ref(),
        ],
        bump,
        constraint = exec.instruction.is_some()
    )]
    pub exec: Account<'info, Exec>,
    
    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.authority.as_ref(),
            queue.name.as_bytes(),
        ],
        bump,
    )]
    pub queue: Account<'info, Queue>,

    #[account(mut)]
    pub worker: Signer<'info>,
}

pub fn handler(ctx: Context<QueueCrank>) -> Result<()> {
    // Get accounts
    let exec = &mut ctx.accounts.exec;
    let queue = &ctx.accounts.queue;

    // Crank the queue
    let bump = ctx.bumps.get("queue").unwrap();
    queue.crank(ctx.remaining_accounts, *bump, exec, &queue.instruction)?;
    
    // TODO Pay fees to worker
    // TODO Support exec account resizing for new instruction

    Ok(())
}
