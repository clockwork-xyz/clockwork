use {
    crate::{errors::*, objects::*},
    anchor_lang::{prelude::*, system_program},
    chrono::{DateTime, NaiveDateTime, Utc},
    clockwork_cron::Schedule,
    clockwork_network_program::objects::Pool,
    std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
        str::FromStr,
    },
};

/// Number of lamports to reimburse the worker with after they've submitted a transaction's worth of cranks.
const TRANSACTION_BASE_FEE_REIMBURSEMENT: u64 = 5000;

/// Accounts required by the `queue_crank` instruction.
#[derive(Accounts)]
#[instruction(data_hash: Option<u64>)]
pub struct QueueCrank<'info> {
    /// The active worker pool.
    #[account(address = Pool::pubkey("queue".to_string()))]
    pub pool: Box<Account<'info, Pool>>,

    /// The queue to crank.
    #[account(
        mut,
        seeds = [
            SEED_QUEUE,
            queue.authority.as_ref(),
            queue.id.as_bytes(),
        ],
        bump,
        constraint = !queue.paused @ ClockworkError::PausedQueue
    )]
    pub queue: Box<Account<'info, Queue>>,

    /// The signatory.
    #[account(mut)]
    pub signatory: Signer<'info>,

    /// The Solana system program.
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<QueueCrank>, data_hash: Option<u64>) -> Result<()> {
    // Get accounts
    let _pool = &ctx.accounts.pool;
    let queue = &mut ctx.accounts.queue;
    let signatory = &ctx.accounts.signatory;

    // If this queue does not have a next_instruction, verify the queue's trigger has been met and a new exec_context can be created.
    let current_slot = Clock::get().unwrap().slot;
    if queue.next_instruction.is_none() {
        match queue.trigger.clone() {
            Trigger::Account { pubkey } => {
                // Require the provided data hash is non-null.
                let data_hash = match data_hash {
                    None => return Err(ClockworkError::InvalidQueueState.into()),
                    Some(data_hash) => data_hash,
                };

                // Verify proof that account data has been updated.
                match ctx.remaining_accounts.first() {
                    None => {}
                    Some(account_info) => {
                        // Verify the remaining account is the account this queue is listening for.
                        require!(pubkey.eq(account_info.key), ClockworkError::InvalidTrigger);

                        // Begin computing the data hash of this account.
                        let mut hasher = DefaultHasher::new();
                        let data = &account_info.try_borrow_data().unwrap();
                        data.to_vec().hash(&mut hasher);

                        // Check the exec context for the prior data hash.
                        let expected_data_hash = match queue.exec_context.clone() {
                            None => {
                                // This queue has not begun executing yet.
                                // There is no prior data hash to include in our hash.
                                hasher.finish()
                            }
                            Some(exec_context) => {
                                match exec_context.trigger_context {
                                    TriggerContext::Account {
                                        data_hash: prior_data_hash,
                                    } => {
                                        // Inject the prior data hash as a seed.
                                        prior_data_hash.hash(&mut hasher);
                                        hasher.finish()
                                    }
                                    _ => return Err(ClockworkError::InvalidQueueState.into()),
                                }
                            }
                        };

                        // Verify the data hash provided by the worker is equal to the expected data hash.
                        // This proves the account has been updated since the last crank and the worker has seen the new data.
                        require!(
                            data_hash.eq(&expected_data_hash),
                            ClockworkError::InvalidTrigger
                        );

                        // Set a new exec context with the new data hash and slot number.
                        queue.exec_context = Some(ExecContext {
                            cranks_since_reimbursement: 0,
                            cranks_since_slot: 0,
                            last_crank_at: current_slot,
                            trigger_context: TriggerContext::Account { data_hash },
                        })
                    }
                }
            }
            Trigger::Cron {
                schedule,
                skippable,
            } => {
                // Get the reference timestamp for calculating the queue's scheduled target timestamp.
                let reference_timestamp = match queue.exec_context.clone() {
                    None => queue.created_at.unix_timestamp,
                    Some(exec_context) => match exec_context.trigger_context {
                        TriggerContext::Cron { started_at } => started_at,
                        _ => return Err(ClockworkError::InvalidQueueState.into()),
                    },
                };

                // Verify the current timestamp is greater than or equal to the threshold timestamp.
                let current_timestamp = Clock::get().unwrap().unix_timestamp;
                let threshold_timestamp = next_timestamp(reference_timestamp, schedule.clone())
                    .ok_or(ClockworkError::InvalidTrigger)?;
                require!(
                    current_timestamp >= threshold_timestamp,
                    ClockworkError::InvalidTrigger
                );

                // If the schedule is marked as skippable, set the started_at of the exec context
                // to be the threshold moment just before the current timestamp.
                let started_at = if skippable && current_timestamp > threshold_timestamp {
                    prev_timestamp(current_timestamp, schedule)
                        .ok_or(ClockworkError::InvalidTrigger)?
                } else {
                    threshold_timestamp
                };

                // Set the exec context.
                queue.exec_context = Some(ExecContext {
                    cranks_since_reimbursement: 0,
                    cranks_since_slot: 0,
                    last_crank_at: current_slot,
                    trigger_context: TriggerContext::Cron { started_at },
                });
            }
            Trigger::Immediate => {
                // Set the exec context.
                require!(
                    queue.exec_context.is_none(),
                    ClockworkError::InvalidQueueState
                );
                queue.exec_context = Some(ExecContext {
                    cranks_since_reimbursement: 0,
                    cranks_since_slot: 0,
                    last_crank_at: current_slot,
                    trigger_context: TriggerContext::Immediate,
                });
            }
        }
    }

    // If the rate limit has been met, exit early.
    match queue.exec_context {
        None => return Err(ClockworkError::InvalidQueueState.into()),
        Some(exec_context) => {
            if exec_context.last_crank_at == Clock::get().unwrap().slot
                && exec_context.cranks_since_slot >= queue.rate_limit
            {
                return Err(ClockworkError::RateLimitExeceeded.into());
            }
        }
    }

    // Crank the queue
    let bump = ctx.bumps.get("queue").unwrap();
    queue.crank(ctx.remaining_accounts, *bump, signatory)?;

    // TODO If worker is in the pool, pay automation fees.
    //      CPI into the network program to pay the fees.
    //
    // let is_authorized_worker = pool.clone().into_inner().workers.contains(&signatory.key());
    // if is_authorized_worker {
    //     fee.escrow_balance(queue.fee, queue)?;
    // } else {
    //     fee.escrow_withholding(queue.fee, queue)?;
    // }

    // If the queue has no more work or the number of cranks since the last payout has reached the rate limit,
    // reimburse the worker for the transaction base fee.
    match queue.exec_context {
        None => return Err(ClockworkError::InvalidQueueState.into()),
        Some(exec_context) => {
            if queue.next_instruction.is_none()
                || exec_context.cranks_since_reimbursement >= queue.rate_limit
            {
                // Pay reimbursment for base transaction fee
                **queue.to_account_info().try_borrow_mut_lamports()? = queue
                    .to_account_info()
                    .lamports()
                    .checked_sub(TRANSACTION_BASE_FEE_REIMBURSEMENT)
                    .unwrap();
                **signatory.to_account_info().try_borrow_mut_lamports()? = signatory
                    .to_account_info()
                    .lamports()
                    .checked_add(TRANSACTION_BASE_FEE_REIMBURSEMENT)
                    .unwrap();

                // Update the exec context to mark that a reimbursement happened this slot.
                queue.exec_context = Some(ExecContext {
                    cranks_since_reimbursement: 0,
                    ..exec_context
                })
            }
        }
    }

    Ok(())
}

fn next_timestamp(after: i64, schedule: String) -> Option<i64> {
    Schedule::from_str(&schedule)
        .unwrap()
        .next_after(&DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(after, 0),
            Utc,
        ))
        .take()
        .map(|datetime| datetime.timestamp())
}

fn prev_timestamp(before: i64, schedule: String) -> Option<i64> {
    Schedule::from_str(&schedule)
        .unwrap()
        .prev_before(&DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(before, 0),
            Utc,
        ))
        .take()
        .map(|datetime| datetime.timestamp())
}
