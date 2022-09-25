use {
    crate::{errors::*, state::*},
    anchor_lang::{prelude::*, system_program},
    chrono::{DateTime, NaiveDateTime, Utc},
    clockwork_cron::Schedule,
    clockwork_pool::state::Pool,
    std::{collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, mem::size_of, str::FromStr},
};

#[derive(Accounts)]
#[instruction(data_hash: Option<u64>)]
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

pub fn handler(ctx: Context<QueueCrank>, data_hash: Option<u64>) -> Result<()> {
    // Get accounts
    let config = &ctx.accounts.config;
    let fee = &mut ctx.accounts.fee;
    let pool = &ctx.accounts.pool;
    let queue = &mut ctx.accounts.queue;
    let worker = &ctx.accounts.worker;

    // If this queue does not have a next_instruction, verify the queue's trigger has been met and a new exec_context can be created.
    if queue.next_instruction.is_none() {
        match queue.trigger.clone() {
            Trigger::Account { pubkey } => {
                // Require the provided data_hash is non-null.
                let data_hash = match data_hash {
                    None => return Err(ClockworkError::InvalidQueueState.into()),
                    Some(data_hash) => data_hash
                };

                // Verify proof that account data has been updated.
                match ctx.remaining_accounts.first() {
                    None => {},
                    Some(account_info) => {
                        // Sanity check on account info pubkey.
                        require!(pubkey.eq(account_info.key), ClockworkError::InvalidTrigger);

                        // Begin computing the expected data hash.
                        let mut hasher = DefaultHasher::new();
                        let data = &account_info.try_borrow_data().unwrap();
                        data.to_vec().hash(&mut hasher);

                        // Check the exec context for the prior data hash.
                        let expected_data_hash = match queue.exec_context.clone() {
                            None => {
                                // This queue has not begun executing yet. 
                                // There is no prior data hash to inject as a seed for the new one. 
                                hasher.finish()
                            }
                            Some(exec_context) => {
                                match exec_context {
                                    ExecContext::Account { data_hash: prior_data_hash } => {
                                        // Inject the prior data hash as a seed to the new one.
                                        prior_data_hash.hash(&mut hasher);
                                        hasher.finish()

                                    },
                                    _ => return Err(ClockworkError::InvalidQueueState.into())
                                }
                            }
                        };

                        // Verify the data hash provided by the worker is equal to the expected data hash.
                        // This proves the account has been updated since the last crank and the worker has seen the new data.
                        require!(data_hash.eq(&expected_data_hash), ClockworkError::InvalidTrigger);

                        // Set a new exec context with the new data hash and slot number.
                        queue.exec_context = Some(ExecContext::Account { data_hash })
                    }
                }
            }
            Trigger::Cron { schedule } => {
                // Get the reference timestamp for calculating the queue's scheduled target timestamp.
                let reference_timestamp = match queue.exec_context.clone() {
                    None => queue.created_at.unix_timestamp,
                    Some(exec_context) => {
                        match exec_context {
                            ExecContext::Cron { started_at } => started_at,
                            _ => return Err(ClockworkError::InvalidQueueState.into())
                        }
                    }
                };

                // Verify the current time is greater than or equal to the target timestamp.
                let target_timestamp = next_timestamp(reference_timestamp, schedule).ok_or(ClockworkError::InvalidTrigger)?;
                let current_timestamp = Clock::get().unwrap().unix_timestamp;
                require!(current_timestamp >= target_timestamp, ClockworkError::InvalidTrigger);

                // Set the exec context.
                queue.exec_context = Some(ExecContext::Cron { started_at: target_timestamp });
            },
            Trigger::Immediate => {
                // Set the exec context.
                require!(queue.exec_context.is_none(), ClockworkError::InvalidQueueState);
                queue.exec_context = Some(ExecContext::Immediate);
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

