use anchor_lang::{
    prelude::*,
    solana_program::{
        instruction::Instruction,
        program::{get_return_data, invoke_signed},
    },
    AnchorDeserialize, InstructionData,
};
use clockwork_network_program::state::{Fee, Pool, Worker, WorkerAccount};
use clockwork_utils::thread::{SerializableInstruction, ThreadResponse, PAYER_PUBKEY};

use crate::{errors::ClockworkError, state::*};

/// The ID of the pool workers must be a member of to collect fees.
const POOL_ID: u64 = 0;

/// The number of lamports to reimburse the worker with after they've submitted a transaction's worth of exec instructions.
pub const TRANSACTION_BASE_FEE_REIMBURSEMENT: u64 = 5_000;

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
            thread.id.as_slice(),
        ],
        bump = thread.bump,
        constraint = !thread.paused @ ClockworkError::ThreadPaused,
        constraint = thread.next_instruction.is_some(),
        constraint = thread.exec_context.is_some()
    )]
    pub thread: Box<Account<'info, Thread>>,

    /// The worker.
    #[account(address = worker.pubkey())]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<ThreadExec>) -> Result<()> {
    // Get accounts
    let clock = Clock::get().unwrap();
    let fee = &mut ctx.accounts.fee;
    let pool = &ctx.accounts.pool;
    let signatory = &mut ctx.accounts.signatory;
    let thread = &mut ctx.accounts.thread;
    let worker = &ctx.accounts.worker;

    // If the rate limit has been met, exit early.
    if thread.exec_context.unwrap().last_exec_at == clock.slot
        && thread.exec_context.unwrap().execs_since_slot >= thread.rate_limit
    {
        return Err(ClockworkError::RateLimitExeceeded.into());
    }

    // Record the worker's lamports before invoking inner ixs.
    let signatory_lamports_pre = signatory.lamports();

    // Get the instruction to execute.
    // We have already verified that it is not null during account validation.
    let instruction: &mut SerializableInstruction = &mut thread.next_instruction.clone().unwrap();

    // Inject the signatory's pubkey for the Clockwork payer ID.
    for acc in instruction.accounts.iter_mut() {
        if acc.pubkey.eq(&PAYER_PUBKEY) {
            acc.pubkey = signatory.key();
        }
    }

    // Invoke the provided instruction.
    invoke_signed(
        &Instruction::from(&*instruction),
        ctx.remaining_accounts,
        &[&[
            SEED_THREAD,
            thread.authority.as_ref(),
            thread.id.as_slice(),
            &[thread.bump],
        ]],
    )?;

    // Verify the inner instruction did not write data to the signatory address.
    require!(signatory.data_is_empty(), ClockworkError::UnauthorizedWrite);

    // Parse the thread response
    let thread_response: Option<ThreadResponse> = match get_return_data() {
        None => None,
        Some((program_id, return_data)) => {
            require!(
                program_id.eq(&instruction.program_id),
                ClockworkError::InvalidThreadResponse
            );
            ThreadResponse::try_from_slice(return_data.as_slice()).ok()
        }
    };

    // Grab the next instruction from the thread response.
    let mut close_to = None;
    let mut next_instruction = None;
    if let Some(thread_response) = thread_response {
        close_to = thread_response.close_to;
        next_instruction = thread_response.dynamic_instruction;

        // Update the trigger.
        if let Some(trigger) = thread_response.trigger {
            require!(
                std::mem::discriminant(&thread.trigger) == std::mem::discriminant(&trigger),
                ClockworkError::InvalidTriggerVariant
            );
            thread.trigger = trigger.clone();

            // If the user updates an account trigger, the trigger context is no longer valid.
            // Here we reset the trigger context to zero to re-prime the trigger.
            thread.exec_context = Some(ExecContext {
                trigger_context: match trigger {
                    Trigger::Account {
                        address: _,
                        offset: _,
                        size: _,
                    } => TriggerContext::Account { data_hash: 0 },
                    _ => thread.exec_context.unwrap().trigger_context,
                },
                ..thread.exec_context.unwrap()
            })
        }
    }

    // If there is no dynamic next instruction, get the next instruction from the instruction set.
    let mut exec_index = thread.exec_context.unwrap().exec_index;
    if next_instruction.is_none() {
        if let Some(ix) = thread.instructions.get((exec_index + 1) as usize) {
            next_instruction = Some(ix.clone());
            exec_index = exec_index + 1;
        }
    }

    // Update the next instruction.
    if let Some(close_to) = close_to {
        thread.next_instruction = Some(
            Instruction {
                program_id: crate::ID,
                accounts: crate::accounts::ThreadDelete {
                    authority: thread.key(),
                    close_to,
                    thread: thread.key(),
                }
                .to_account_metas(Some(true)),
                data: crate::instruction::ThreadDelete {}.data(),
            }
            .into(),
        );
    } else {
        thread.next_instruction = next_instruction;
    }

    // Update the exec context.
    let should_reimburse_transaction = clock.slot > thread.exec_context.unwrap().last_exec_at;
    thread.exec_context = Some(ExecContext {
        exec_index,
        execs_since_slot: if clock.slot == thread.exec_context.unwrap().last_exec_at {
            thread
                .exec_context
                .unwrap()
                .execs_since_slot
                .checked_add(1)
                .unwrap()
        } else {
            1
        },
        last_exec_at: clock.slot,
        ..thread.exec_context.unwrap()
    });

    // Reimbursement signatory for lamports paid during inner ix.
    let signatory_lamports_post = signatory.lamports();
    let mut signatory_reimbursement =
        signatory_lamports_pre.saturating_sub(signatory_lamports_post);
    if should_reimburse_transaction {
        signatory_reimbursement = signatory_reimbursement
            .checked_add(TRANSACTION_BASE_FEE_REIMBURSEMENT)
            .unwrap();
    }
    if signatory_reimbursement.gt(&0) {
        **thread.to_account_info().try_borrow_mut_lamports()? = thread
            .to_account_info()
            .lamports()
            .checked_sub(signatory_reimbursement)
            .unwrap();
        **signatory.to_account_info().try_borrow_mut_lamports()? = signatory
            .to_account_info()
            .lamports()
            .checked_add(signatory_reimbursement)
            .unwrap();
    }

    // If the worker is in the pool, debit from the thread account and payout to the worker's fee account.
    if pool.clone().into_inner().workers.contains(&worker.key()) {
        **thread.to_account_info().try_borrow_mut_lamports()? = thread
            .to_account_info()
            .lamports()
            .checked_sub(thread.fee)
            .unwrap();
        **fee.to_account_info().try_borrow_mut_lamports()? = fee
            .to_account_info()
            .lamports()
            .checked_add(thread.fee)
            .unwrap();
    }

    Ok(())
}
