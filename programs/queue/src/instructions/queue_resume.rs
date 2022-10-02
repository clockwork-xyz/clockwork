use {crate::objects::*, anchor_lang::prelude::*};

/// Accounts required by the `queue_resume` instruction.
#[derive(Accounts)]
pub struct QueueResume<'info> {
    /// The authority (owner) of the queue.
    #[account()]
    pub authority: Signer<'info>,

    /// The queue to be resumed.
    #[account(
        mut,
        address = queue.pubkey(),
        has_one = authority
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<QueueResume>) -> Result<()> {
    // Get accounts
    let queue = &mut ctx.accounts.queue;

    // Resume the queue
    queue.paused = false;

    // Update the exec context
    match queue.exec_context {
        None => {}
        Some(exec_context) => {
            match exec_context.trigger_context {
                TriggerContext::Account { data_hash: _ } => {
                    // Nothing to do
                }
                TriggerContext::Cron { started_at: _ } => {
                    // Jump ahead to the current timestamp
                    queue.exec_context = Some(ExecContext {
                        trigger_context: TriggerContext::Cron {
                            started_at: Clock::get().unwrap().unix_timestamp,
                        },
                        ..exec_context
                    });
                }
                TriggerContext::Immediate => {
                    // Nothing to do
                }
            }
        }
    }

    Ok(())
}
