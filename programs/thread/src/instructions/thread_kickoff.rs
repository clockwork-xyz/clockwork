use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    str::FromStr,
};

use anchor_lang::prelude::*;
use chrono::{DateTime, NaiveDateTime, Utc};
use clockwork_cron::Schedule;
use clockwork_network_program::state::{Worker, WorkerAccount};
use clockwork_utils::thread::Trigger;
use pyth_sdk_solana::load_price_feed_from_account_info;

use crate::{errors::*, state::*};

use super::TRANSACTION_BASE_FEE_REIMBURSEMENT;

/// Accounts required by the `thread_kickoff` instruction.
#[derive(Accounts)]
pub struct ThreadKickoff<'info> {
    /// The signatory.
    #[account(mut)]
    pub signatory: Signer<'info>,

    /// The thread to kickoff.
    #[account(
        mut,
        seeds = [
            SEED_THREAD,
            thread.authority.as_ref(),
            thread.id.as_slice(),
        ],
        bump = thread.bump,
        constraint = !thread.paused @ ClockworkError::ThreadPaused,
        constraint = thread.next_instruction.is_none() @ ClockworkError::ThreadBusy,
    )]
    pub thread: Box<Account<'info, Thread>>,

    /// The worker.
    #[account(address = worker.pubkey())]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<ThreadKickoff>) -> Result<()> {
    // Get accounts.
    let signatory = &mut ctx.accounts.signatory;
    let thread = &mut ctx.accounts.thread;
    let clock = Clock::get().unwrap();

    match thread.trigger.clone() {
        Trigger::Account {
            address,
            offset,
            size,
        } => {
            // Verify proof that account data has been updated.
            match ctx.remaining_accounts.first() {
                None => {
                    return Err(ClockworkError::TriggerConditionFailed.into());
                }
                Some(account_info) => {
                    // Verify the remaining account is the account this thread is listening for.
                    require!(
                        address.eq(account_info.key),
                        ClockworkError::TriggerConditionFailed
                    );

                    // Begin computing the data hash of this account.
                    let mut hasher = DefaultHasher::new();
                    let data = &account_info.try_borrow_data().unwrap();
                    let offset = offset as usize;
                    let range_end = offset.checked_add(size as usize).unwrap() as usize;
                    if data.len().gt(&range_end) {
                        data[offset..range_end].hash(&mut hasher);
                    } else {
                        data[offset..].hash(&mut hasher)
                    }
                    let data_hash = hasher.finish();

                    // Verify the data hash is different than the prior data hash.
                    if let Some(exec_context) = thread.exec_context {
                        match exec_context.trigger_context {
                            TriggerContext::Account {
                                data_hash: prior_data_hash,
                            } => {
                                require!(
                                    data_hash.ne(&prior_data_hash),
                                    ClockworkError::TriggerConditionFailed
                                )
                            }
                            _ => return Err(ClockworkError::InvalidThreadState.into()),
                        }
                    }

                    // Set a new exec context with the new data hash and slot number.
                    thread.exec_context = Some(ExecContext {
                        exec_index: 0,
                        execs_since_reimbursement: 0,
                        execs_since_slot: 0,
                        last_exec_at: clock.slot,
                        trigger_context: TriggerContext::Account { data_hash },
                    })
                }
            }
        }
        Trigger::Cron {
            schedule,
            skippable,
        } => {
            // Get the reference timestamp for calculating the thread's scheduled target timestamp.
            let reference_timestamp = match thread.exec_context.clone() {
                None => thread.created_at.unix_timestamp,
                Some(exec_context) => match exec_context.trigger_context {
                    TriggerContext::Cron { started_at } => started_at,
                    _ => return Err(ClockworkError::InvalidThreadState.into()),
                },
            };

            // Verify the current timestamp is greater than or equal to the threshold timestamp.
            let threshold_timestamp = next_timestamp(reference_timestamp, schedule.clone())
                .ok_or(ClockworkError::TriggerConditionFailed)?;
            require!(
                clock.unix_timestamp.ge(&threshold_timestamp),
                ClockworkError::TriggerConditionFailed
            );

            // If the schedule is marked as skippable, set the started_at of the exec context to be the current timestamp.
            // Otherwise, the exec context must iterate through each scheduled kickoff moment.
            let started_at = if skippable {
                clock.unix_timestamp
            } else {
                threshold_timestamp
            };

            // Set the exec context.
            thread.exec_context = Some(ExecContext {
                exec_index: 0,
                execs_since_reimbursement: 0,
                execs_since_slot: 0,
                last_exec_at: clock.slot,
                trigger_context: TriggerContext::Cron { started_at },
            });
        }
        Trigger::Now => {
            // Set the exec context.
            require!(
                thread.exec_context.is_none(),
                ClockworkError::InvalidThreadState
            );
            thread.exec_context = Some(ExecContext {
                exec_index: 0,
                execs_since_reimbursement: 0,
                execs_since_slot: 0,
                last_exec_at: clock.slot,
                trigger_context: TriggerContext::Now,
            });
        }
        Trigger::Slot { slot } => {
            require!(clock.slot.ge(&slot), ClockworkError::TriggerConditionFailed);
            thread.exec_context = Some(ExecContext {
                exec_index: 0,
                execs_since_reimbursement: 0,
                execs_since_slot: 0,
                last_exec_at: clock.slot,
                trigger_context: TriggerContext::Slot { started_at: slot },
            });
        }
        Trigger::Epoch { epoch } => {
            require!(
                clock.epoch.ge(&epoch),
                ClockworkError::TriggerConditionFailed
            );
            thread.exec_context = Some(ExecContext {
                exec_index: 0,
                execs_since_reimbursement: 0,
                execs_since_slot: 0,
                last_exec_at: clock.slot,
                trigger_context: TriggerContext::Epoch { started_at: epoch },
            })
        }
        Trigger::Timestamp { unix_ts } => {
            require!(
                clock.unix_timestamp.ge(&unix_ts),
                ClockworkError::TriggerConditionFailed
            );
            thread.exec_context = Some(ExecContext {
                exec_index: 0,
                execs_since_reimbursement: 0,
                execs_since_slot: 0,
                last_exec_at: clock.slot,
                trigger_context: TriggerContext::Timestamp {
                    started_at: unix_ts,
                },
            })
        }
        Trigger::Pyth {
            price_feed: price_feed_pubkey,
            equality,
            limit,
        } => {
            // Verify price limit has been reached.
            match ctx.remaining_accounts.first() {
                None => {
                    return Err(ClockworkError::TriggerConditionFailed.into());
                }
                Some(account_info) => {
                    require!(
                        price_feed_pubkey.eq(account_info.key),
                        ClockworkError::TriggerConditionFailed
                    );
                    const STALENESS_THRESHOLD: u64 = 60; // staleness threshold in seconds
                    let price_feed = load_price_feed_from_account_info(account_info).unwrap();
                    let current_timestamp = Clock::get()?.unix_timestamp;
                    let current_price = price_feed
                        .get_price_no_older_than(current_timestamp, STALENESS_THRESHOLD)
                        .unwrap();
                    match equality {
                        Equality::GreaterThanOrEqual => {
                            require!(
                                current_price.price.ge(&limit),
                                ClockworkError::TriggerConditionFailed
                            );
                            thread.exec_context = Some(ExecContext {
                                exec_index: 0,
                                execs_since_reimbursement: 0,
                                execs_since_slot: 0,
                                last_exec_at: clock.slot,
                                trigger_context: TriggerContext::Pyth {
                                    price: current_price.price,
                                },
                            });
                        }
                        Equality::LessThanOrEqual => {
                            require!(
                                current_price.price.le(&limit),
                                ClockworkError::TriggerConditionFailed
                            );
                            thread.exec_context = Some(ExecContext {
                                exec_index: 0,
                                execs_since_reimbursement: 0,
                                execs_since_slot: 0,
                                last_exec_at: clock.slot,
                                trigger_context: TriggerContext::Pyth {
                                    price: current_price.price,
                                },
                            });
                        }
                    }
                }
            }
        }
    }

    // If we make it here, the trigger is active. Update the next instruction and be done.
    if let Some(kickoff_instruction) = thread.instructions.first() {
        thread.next_instruction = Some(kickoff_instruction.clone());
    }

    // Realloc the thread account
    thread.realloc()?;

    // Reimburse signatory for transaction fee.
    **thread.to_account_info().try_borrow_mut_lamports()? = thread
        .to_account_info()
        .lamports()
        .checked_sub(TRANSACTION_BASE_FEE_REIMBURSEMENT)
        .unwrap();
    **signatory.to_account_info().try_borrow_mut_lamports()? = signatory
        .to_account_info()
        .lamports()
        .checked_add(TRANSACTION_BASE_FEE_REIMBURSEMENT)
        .unwrap();

    Ok(())
}

fn next_timestamp(after: i64, schedule: String) -> Option<i64> {
    Schedule::from_str(&schedule)
        .unwrap()
        .next_after(&DateTime::<Utc>::from_naive_utc_and_offset(
            NaiveDateTime::from_timestamp_opt(after, 0).unwrap(),
            Utc,
        ))
        .take()
        .map(|datetime| datetime.timestamp())
}
