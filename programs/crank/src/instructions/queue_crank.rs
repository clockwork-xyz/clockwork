use {
    crate::{errors::*, state::*},
    anchor_lang::prelude::*,
    chrono::{DateTime, NaiveDateTime, Utc},
    clockwork_cron::Schedule,
    std::str::FromStr,
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
    )]
    pub queue: Account<'info, Queue>,

    #[account(mut)]
    pub worker: Signer<'info>,
}

pub fn handler(ctx: Context<QueueCrank>) -> Result<()> {
    // Get accounts
    let queue = &mut ctx.accounts.queue;
    let worker = &ctx.accounts.worker;

    // If this queue does not have a next_instruction, verify the queue's trigger has been met and a new exec_context can be created.
    if queue.next_instruction.is_none() {
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
                msg!("reference_timestamp: {:#?} schedule: {:#?}", reference_timestamp, schedule);

                // Verify the current time is greater than or equal to the target timestamp.
                let target_timestamp = next_moment(reference_timestamp, schedule).ok_or(ClockworkError::TriggerNotMet)?;
                let current_timestamp = Clock::get().unwrap().unix_timestamp;

                msg!("current_timestamp: {:#?} target_timestamp: {:#?}", current_timestamp, target_timestamp);
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
    }

    // Crank the queue
    let bump = ctx.bumps.get("queue").unwrap();
    queue.crank(ctx.remaining_accounts, *bump, worker)?;
    
    // TODO Pay fees to worker
    // TODO Dynamically resize queue account, if needed

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
