use {
    crate::state::*,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct QueueResume<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.authority.key().as_ref(),
            queue.name.as_bytes(),
        ],
        bump,
        has_one = authority
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<QueueResume>) -> Result<()> {
    // Get accounts
    let queue = &mut ctx.accounts.queue;
    
    // Resume the queue
    queue.is_paused = false;

    // Update the exec context
    match queue.exec_context {
        None => {}
        Some(exec_context) => {
            match exec_context {
                ExecContext::Cron { started_at: _ } => {
                    // Jump ahead to the current timestamp
                    queue.exec_context = Some(ExecContext::Cron { started_at: Clock::get().unwrap().unix_timestamp });
                }
                ExecContext::Immediate => {
                    // Nothing to do
                }
            }
        }
    }

    Ok(())
}
