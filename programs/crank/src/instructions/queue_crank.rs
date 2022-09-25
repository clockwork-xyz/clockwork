use {
    crate::{errors::*, state::*},
    anchor_lang::{prelude::*, system_program},
    chrono::{DateTime, NaiveDateTime, Utc},
    clockwork_cron::Schedule,
    clockwork_pool::state::Pool,
    std::{mem::size_of, str::FromStr},
};

#[derive(Accounts)]
pub struct QueueCrank<'info> {
    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init_if_needed,
        seeds = [
            SEED_FEE,
            worker.key().as_ref()
        ],
        bump,
        payer = worker,
        space = 8 + size_of::<Fee>(),
    )]
    pub fee: Box<Account<'info, Fee>>,

    #[account(address = config.worker_pool)]
    pub pool: Box<Account<'info, Pool>>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.authority.as_ref(),
            queue.id.as_bytes(),
        ],
        bump,
        constraint = !queue.is_paused @ ClockworkError::PausedQueue
    )]
    pub queue: Box<Account<'info, Queue>>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub worker: Signer<'info>,
}

pub fn handler(ctx: Context<QueueCrank>) -> Result<()> {
    // Get accounts
    let config = &ctx.accounts.config;
    let fee = &mut ctx.accounts.fee;
    let pool = &ctx.accounts.pool;
    let queue = &mut ctx.accounts.queue;
    let worker = &ctx.accounts.worker;

    // If this queue does not have a next_instruction, verify the queue's trigger has been met and a new exec_context can be created.
    let current_slot = Clock::get().unwrap().slot;
    if queue.next_instruction.is_none() {
        match queue.trigger.clone() {
            Trigger::Cron { schedule } => {
                // Get the reference timestamp for calculating the queue's scheduled target timestamp.
                let reference_timestamp = match queue.exec_context.clone() {
                    None => queue.created_at.unix_timestamp,
                    Some(exec_context) => {
                        match exec_context.trigger_context {
                            TriggerContext::Cron { started_at } => started_at,
                            _ => return Err(ClockworkError::InvalidQueueState.into())
                        }
                    }
                };

                // Verify the current time is greater than or equal to the target timestamp.
                let target_timestamp = next_timestamp(reference_timestamp, schedule).ok_or(ClockworkError::InvalidTrigger)?;
                let current_timestamp = Clock::get().unwrap().unix_timestamp;
                require!(current_timestamp >= target_timestamp, ClockworkError::InvalidTrigger);

                // Set the exec context.
                queue.exec_context = Some(ExecContext {
                    crank_count: 0,
                    last_crank_at: current_slot,
                    trigger_context: TriggerContext::Cron { started_at: target_timestamp }
                });
            },
            Trigger::Immediate => {
                // Set the exec context.
                require!(queue.exec_context.is_none(), ClockworkError::InvalidQueueState);
                queue.exec_context = Some(ExecContext {
                    crank_count: 0,
                    last_crank_at: current_slot,
                    trigger_context: TriggerContext::Immediate,
                });
            },
        }
    }

    // Crank the queue
    let bump = ctx.bumps.get("queue").unwrap();
    queue.crank(ctx.remaining_accounts, *bump, worker)?;
    
    // If worker is in the pool, pay automation fees.
    let is_authorized_worker = pool.clone().into_inner().workers.contains(&worker.key());
    if is_authorized_worker {
        fee.pay_to_worker(config.crank_fee, queue)?;
    } else {
        fee.pay_to_admin(config.crank_fee, queue)?;
    }

    // Increment the crank count 
    match queue.exec_context {
        None => return Err(ClockworkError::InvalidQueueState.into()),
        Some(exec_context) => {
            queue.exec_context = Some(ExecContext {
                crank_count: if exec_context.last_crank_at == current_slot {
                    exec_context.crank_count.checked_add(1).unwrap()
                } else {
                    1
                },
                last_crank_at: current_slot,
                trigger_context: exec_context.trigger_context,
            });
        }
    }

    Ok(())
}


fn next_timestamp(after: i64, schedule: String) -> Option<i64> {
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

