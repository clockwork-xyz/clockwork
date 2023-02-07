use {crate::state::*, anchor_lang::prelude::*};

/// Accounts required by the `automation_resume` instruction.
#[derive(Accounts)]
pub struct AutomationResume<'info> {
    /// The authority (owner) of the automation.
    #[account()]
    pub authority: Signer<'info>,

    /// The automation to be resumed.
    #[account(
        mut,
        seeds = [
            SEED_AUTOMATION,
            automation.authority.as_ref(),
            automation.id.as_slice(),
        ],
        bump = automation.bump,
        has_one = authority
    )]
    pub automation: Account<'info, Automation>,
}

pub fn handler(ctx: Context<AutomationResume>) -> Result<()> {
    // Get accounts
    let automation = &mut ctx.accounts.automation;

    // Resume the automation
    automation.paused = false;

    // Update the exec context
    match automation.exec_context {
        None => {}
        Some(exec_context) => {
            match exec_context.trigger_context {
                TriggerContext::Account { data_hash: _ } => {
                    // Nothing to do
                }
                TriggerContext::Cron { started_at: _ } => {
                    // Jump ahead to the current timestamp
                    automation.exec_context = Some(ExecContext {
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
