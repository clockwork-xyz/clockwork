use {
    crate::{errors::*, objects::*},
    anchor_lang::prelude::*,
    clockwork_network_program::objects::{Fee, Penalty, Pool, Worker, WorkerAccount},
};

/// The ID of the pool workers must be a member of to collect fees.
const POOL_ID: u64 = 0;

/// Number of lamports to reimburse the worker with after they've submitted a transaction's worth of cranks.
const TRANSACTION_BASE_FEE_REIMBURSEMENT: u64 = 5_000;

/// Accounts required by the `queue_crank` instruction.
#[derive(Accounts)]
#[instruction(data_hash: Option<u64>)]
pub struct QueueCrank<'info> {
    /// The worker's fee account.
    #[account(
        mut,
        seeds = [
            clockwork_network_program::objects::SEED_FEE,
            worker.key().as_ref(),
        ],
        bump,
        seeds::program = clockwork_network_program::ID,
        has_one = worker,
    )]
    pub fee: Account<'info, Fee>,

    /// The worker's penalty account.
    #[account(
        mut,
        seeds = [
            clockwork_network_program::objects::SEED_PENALTY,
            worker.key().as_ref(),
        ],
        bump,
        seeds::program = clockwork_network_program::ID,
        has_one = worker,
    )]
    pub penalty: Account<'info, Penalty>,

    /// The active worker pool.
    #[account(address = Pool::pubkey(POOL_ID))]
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
        constraint = !queue.paused @ ClockworkError::QueuePaused
    )]
    pub queue: Box<Account<'info, Queue>>,

    /// The signatory.
    #[account(mut)]
    pub signatory: Signer<'info>,

    /// The worker.
    #[account(
        address = worker.pubkey(),
        has_one = signatory
    )]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<QueueCrank>, data_hash: Option<u64>) -> Result<()> {
    // Get accounts
    let fee = &mut ctx.accounts.fee;
    let penalty = &ctx.accounts.penalty;
    let pool = &ctx.accounts.pool;
    let queue = &mut ctx.accounts.queue;
    let signatory = &ctx.accounts.signatory;
    let worker = &ctx.accounts.worker;

    if queue.next_instruction.is_none() {
        // If this queue does not have a next_instruction, verify the queue's trigger condition is active.
        queue.verify_trigger(data_hash, ctx.remaining_accounts)?;
    } else {
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

        // Debit the crank fee from the queue account.
        **queue.to_account_info().try_borrow_mut_lamports()? = queue
            .to_account_info()
            .lamports()
            .checked_sub(queue.fee)
            .unwrap();

        // If the worker is in the pool, pay fee to the worker's fee account.
        // Otherwise, pay fee to the worker's penalty account.
        if pool.clone().into_inner().workers.contains(&worker.key()) {
            **fee.to_account_info().try_borrow_mut_lamports()? = fee
                .to_account_info()
                .lamports()
                .checked_add(queue.fee)
                .unwrap();
        } else {
            **penalty.to_account_info().try_borrow_mut_lamports()? = penalty
                .to_account_info()
                .lamports()
                .checked_add(queue.fee)
                .unwrap();
        }

        // If the queue has no more work or the number of cranks since the last payout has reached the rate limit,
        // reimburse the worker for the transaction base fee.
        match queue.exec_context {
            None => {
                return Err(ClockworkError::InvalidQueueState.into());
            }
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
                    });
                }
            }
        }
    }

    Ok(())
}
