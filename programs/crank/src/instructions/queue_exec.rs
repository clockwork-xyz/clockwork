use {
    crate::{errors::*, state::*},
    anchor_lang::prelude::*,
    chrono::{DateTime, NaiveDateTime, Utc},
    clockwork_cron::Schedule,
    std::str::FromStr,
};

#[derive(Accounts)]
pub struct QueueExec<'info> {
    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.authority.as_ref(),
            queue.name.as_bytes(),
        ],
        bump,
        constraint = queue.next_instruction.is_none()
    )]
    pub queue: Account<'info, Queue>,

    #[account(mut)]
    pub worker: Signer<'info>,
}

pub fn handler(ctx: Context<QueueExec>) -> Result<()> {

    // Get accounts
    let queue = &mut ctx.accounts.queue;

    // Verify the queue's trigger has been met and an exec has not already been started for this event.
    match queue.trigger.clone() {
        Trigger::Cron { schedule } => {
            // Get the reference timestamp for calculating the queue's scheduled target timestamp.
            let reference_timestamp = match queue.exec_context.clone() {
                None => queue.created_at.unix_timestamp,
                Some(exec_context) => {
                    match exec_context {
                        ExecContext::Cron { last_exec_at } => last_exec_at,
                        _ => return Err(ClockworkError::InvalidExecContext.into())
                    }
                }
            };

            // Verify the current time is greater than or equal to the target timestamp.
            let target_timestamp = next_moment(reference_timestamp, schedule).ok_or(ClockworkError::TriggerNotMet)?;
            let current_timestamp = Clock::get().unwrap().unix_timestamp;
            require!(current_timestamp >= target_timestamp, ClockworkError::TriggerNotMet);

            // Set the exec context.
            queue.exec_context = Some(ExecContext::Cron { last_exec_at: target_timestamp });
        },
        Trigger::Immediate => {
            // Set the exec context.
            require!(queue.exec_context.is_none(), ClockworkError::QueueAlreadyStarted);
            queue.exec_context = Some(ExecContext::Immediate);
        },
    }

    // Crank the queue.
    let bump = ctx.bumps.get("queue").unwrap();
    let instruction = &queue.clone().first_instruction;
    queue.crank(ctx.remaining_accounts, *bump, instruction)?;

    Ok(())
}

fn next_moment(after: i64, schedule: String) -> Option<i64> {
    Schedule::from_str(&schedule)
        .unwrap()
        .after(&DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(after, 0),
            Utc,
        ))
        .take(1)
        .next()
        .map(|datetime| datetime.timestamp())
}
