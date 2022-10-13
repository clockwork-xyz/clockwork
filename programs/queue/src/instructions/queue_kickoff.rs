use {
    crate::{errors::*, objects::*},
    anchor_lang::prelude::*,
    clockwork_network_program::objects::{Worker, WorkerAccount},
};

/// Accounts required by the `queue_crank` instruction.
#[derive(Accounts)]
#[instruction(data_hash: Option<u64>)]
pub struct QueueKickoff<'info> {
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
        constraint = queue.next_instruction.is_none()
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

pub fn handler(ctx: Context<QueueKickoff>, data_hash: Option<u64>) -> Result<()> {
    // Get accounts
    let queue = &mut ctx.accounts.queue;

    // If this queue does not have a next_instruction, verify the queue's trigger condition is active.
    queue.kickoff(data_hash, ctx.remaining_accounts)?;

    Ok(())
}
