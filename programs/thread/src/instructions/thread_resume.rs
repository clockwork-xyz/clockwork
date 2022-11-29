use {crate::state::*, anchor_lang::prelude::*};

/// Accounts required by the `thread_resume` instruction.
#[derive(Accounts)]
pub struct ThreadResume<'info> {
    /// The authority (owner) of the thread.
    #[account()]
    pub authority: Signer<'info>,

    /// The thread to be resumed.
    #[account(
        mut,
        seeds = [
            SEED_THREAD,
            thread.authority.as_ref(),
            thread.id.as_bytes(),
        ],
        bump,
        has_one = authority
    )]
    pub thread: Account<'info, Thread>,
}

pub fn handler(ctx: Context<ThreadResume>) -> Result<()> {
    // Get accounts
    let thread = &mut ctx.accounts.thread;

    // Resume the thread
    thread.paused = false;

    // Update the exec context
    match thread.exec_context {
        None => {}
        Some(exec_context) => {
            match exec_context.trigger_context {
                TriggerContext::Account { data_hash: _ } => {
                    // Nothing to do
                }
                TriggerContext::Cron { started_at: _ } => {
                    // Jump ahead to the current timestamp
                    thread.exec_context = Some(ExecContext {
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
