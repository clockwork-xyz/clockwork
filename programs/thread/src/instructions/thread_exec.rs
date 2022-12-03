use {
    crate::{errors::*, state::*},
    anchor_lang::prelude::*,
    clockwork_network_program::state::{Fee, Penalty, Pool, Worker, WorkerAccount},
};

/// The ID of the pool workers must be a member of to collect fees.
const POOL_ID: u64 = 0;

/// Accounts required by the `thread_exec` instruction.
#[derive(Accounts)]
pub struct ThreadExec<'info> {
    /// The worker's fee account.
    #[account(
        mut,
        seeds = [
            clockwork_network_program::state::SEED_FEE,
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
            clockwork_network_program::state::SEED_PENALTY,
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

    /// The signatory.
    #[account(mut)]
    pub signatory: Signer<'info>,

    /// The thread to execute.
    #[account(
        mut,
        seeds = [
            SEED_THREAD,
            thread.authority.as_ref(),
            thread.id.as_bytes(),
        ],
        bump,
        constraint = !thread.paused @ ClockworkError::ThreadPaused,
        constraint = thread.next_instruction.is_some()
    )]
    pub thread: Box<Account<'info, Thread>>,

    /// The worker.
    #[account(address = worker.pubkey())]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<ThreadExec>) -> Result<()> {
    // Get accounts
    let fee = &mut ctx.accounts.fee;
    let penalty = &mut ctx.accounts.penalty;
    let pool = &ctx.accounts.pool;
    let signatory = &mut ctx.accounts.signatory;
    let thread = &mut ctx.accounts.thread;
    let worker = &ctx.accounts.worker;

    // If the rate limit has been met, exit early.
    match thread.exec_context {
        None => return Err(ClockworkError::InvalidThreadState.into()),
        Some(exec_context) => {
            if exec_context.last_exec_at == Clock::get().unwrap().slot
                && exec_context.execs_since_slot >= thread.rate_limit
            {
                return Err(ClockworkError::RateLimitExeceeded.into());
            }
        }
    }

    // Execute the thread
    let bump = ctx.bumps.get("thread").unwrap();
    thread.exec(
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
