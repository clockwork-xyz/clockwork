use {
    crate::{errors::*, objects::*},
    anchor_lang::prelude::*,
    clockwork_network_program::objects::{Fee, Penalty, Pool, Worker, WorkerAccount},
};

/// The ID of the pool workers must be a member of to collect fees.
const POOL_ID: u64 = 0;

/// Accounts required by the `queue_crank` instruction.
#[derive(Accounts)]
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
        constraint = !queue.paused @ ClockworkError::QueuePaused,
        constraint = queue.next_instruction.is_some()
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

pub fn handler(ctx: Context<QueueCrank>) -> Result<()> {
    // Get accounts
    let fee = &mut ctx.accounts.fee;
    let penalty = &mut ctx.accounts.penalty;
    let pool = &ctx.accounts.pool;
    let queue = &mut ctx.accounts.queue;
    let signatory = &mut ctx.accounts.signatory;
    let worker = &ctx.accounts.worker;

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
    queue.crank(
        ctx.remaining_accounts,
        *bump,
        fee,
        penalty,
        pool,
        signatory,
        worker,
    )?;

    Ok(())
}
