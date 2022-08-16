use {
    crate::{errors::*, state::*},
    anchor_lang::{prelude::*, solana_program::system_program},
    chrono::{DateTime, NaiveDateTime, Utc},
    clockwork_cron::Schedule,
    std::{mem::{size_of, size_of_val}, str::FromStr},
};

#[derive(Accounts)]
pub struct QueueStart<'info> {
    #[account(
        init,
        seeds = [
            SEED_QUEUE, 
            queue.key().as_ref(),
            queue.exec_count.to_be_bytes().as_ref(),
        ],
        bump,
        payer = worker,
        space = 8 + size_of::<Exec>() + size_of_val(&queue.instruction),
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

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub worker: Signer<'info>,
}

pub fn handler(ctx: Context<QueueStart>) -> Result<()> {

    // Get accounts
    let exec = &mut ctx.accounts.exec;
    let queue = &mut ctx.accounts.queue;

    // Parse the queue's last_exec account, if expected.
    let (last_exec, remaining_accounts) = match queue.last_exec {
        None => (None, ctx.remaining_accounts),
        Some(last_exec_pubkey) => {
            match ctx.remaining_accounts.split_first() {
                None => (None, ctx.remaining_accounts),
                Some((acc_info, remaining_accounts)) => {
                    require!(acc_info.key().eq(&last_exec_pubkey), ClockworkError::InvalidExecContext);
                    (Some(Account::<'_, Exec>::try_from(acc_info).unwrap()), remaining_accounts)
                }
            }
        }
    };

    // Verify the queue's trigger has been met and an exec has not already been started for this event.
    match queue.trigger.clone() {
        Trigger::Cron { schedule } => {
            match last_exec {
                None => {}, 
                Some(last_exec) => {
                    match last_exec.context {
                        ExecContext::Cron { unix_timestamp  } => {
                            match next_moment(unix_timestamp, schedule) {
                                Some(target_unix_timestamp) => {
                                    
                                    // Verify the current clock time is equal to or greater than the target timestamp.
                                    require!(Clock::get().unwrap().unix_timestamp >= target_unix_timestamp, ClockworkError::TriggerNotMet);

                                    // Initialize the new exec account.
                                    let new_exec_context = ExecContext::Cron { unix_timestamp: target_unix_timestamp };
                                    exec.init(new_exec_context, queue)?;
                                }
                                None => return Err(ClockworkError::TriggerNotMet.into())
                            };
                        },
                        _ => return Err(ClockworkError::InvalidExecContext.into())
                    }
                },
            }
        },
        Trigger::Immediate => require!(queue.last_exec == None, ClockworkError::QueueAlreadyStarted),
    }

    // Crank the queue.
    let bump = ctx.bumps.get("queue").unwrap();
    queue.crank(remaining_accounts, *bump, exec, &queue.instruction)?;

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