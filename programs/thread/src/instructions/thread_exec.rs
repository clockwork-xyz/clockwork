use clockwork_utils::{InstructionData, ThreadResponse};

use {
    crate::errors::ClockworkError,
    crate::state::*,
    anchor_lang::{
        prelude::*,
        solana_program::{
            instruction::Instruction,
            program::{get_return_data, invoke_signed},
        },
        AnchorDeserialize,
    },
    clockwork_network_program::state::{Fee, Penalty, Pool, Worker, WorkerAccount},
};

/// The ID of the pool workers must be a member of to collect fees.
const POOL_ID: u64 = 0;

/// The number of lamports to reimburse the worker with after they've submitted a transaction's worth of exec instructions.
const TRANSACTION_BASE_FEE_REIMBURSEMENT: u64 = 5_000;

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
    let fee = &mut ctx.accounts.fee;
    let penalty = &mut ctx.accounts.penalty;
    let pool = &ctx.accounts.pool;
    let signatory = &mut ctx.accounts.signatory;
    let thread = &mut ctx.accounts.thread;
    let worker = &ctx.accounts.worker;

    // If the rate limit has been met, exit early.
    if thread.exec_context.unwrap().last_exec_at == Clock::get().unwrap().slot
        && thread.exec_context.unwrap().execs_since_slot >= thread.rate_limit
    {
        return Err(ClockworkError::RateLimitExeceeded.into());
    }

    // Record the worker's lamports before invoking inner ixs.
    let signatory_lamports_pre = signatory.lamports();

    // Get the instruction to execute.
    // We have already verified that it is not null during account validation.
    let next_instruction: &Option<InstructionData> = &thread.clone().next_instruction;
    let instruction = next_instruction.as_ref().unwrap();

    // Inject the signatory's pubkey for the Clockwork payer ID.
    let normalized_accounts: &mut Vec<AccountMeta> = &mut vec![];
    instruction.accounts.iter().for_each(|acc| {
        let acc_pubkey = if acc.pubkey == clockwork_utils::PAYER_PUBKEY {
            signatory.key()
        } else {
            acc.pubkey
        };
        normalized_accounts.push(AccountMeta {
            pubkey: acc_pubkey,
            is_signer: acc.is_signer,
            is_writable: acc.is_writable,
        });
    });

    // Invoke the provided instruction.
    invoke_signed(
        &Instruction {
            program_id: instruction.program_id,
            data: instruction.data.clone(),
            accounts: normalized_accounts.to_vec(),
        },
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
    let mut next_instruction = None;
    if let Some(thread_response) = thread_response {
        next_instruction = thread_response.next_instruction;

        // Update the trigger.
        if let Some(trigger) = thread_response.trigger {
            require!(
                std::mem::discriminant(&thread.trigger) == std::mem::discriminant(&trigger),
                ClockworkError::InvalidTriggerVariant
            );
            thread.trigger = trigger;
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
    thread.next_instruction = next_instruction;

    // Update the exec context.
    let current_slot = Clock::get().unwrap().slot;
    thread.exec_context = Some(ExecContext {
        exec_index,
        execs_since_reimbursement: thread
            .exec_context
            .unwrap()
            .execs_since_reimbursement
            .checked_add(1)
            .unwrap(),
        execs_since_slot: if current_slot == thread.exec_context.unwrap().last_exec_at {
            thread
                .exec_context
                .unwrap()
                .execs_since_slot
                .checked_add(1)
                .unwrap()
        } else {
            1
        },
        last_exec_at: current_slot,
        ..thread.exec_context.unwrap()
    });

    // Realloc memory for the thread account.
    thread.realloc()?;

    // Reimbursement signatory for lamports paid during inner ix.
    let signatory_lamports_post = signatory.lamports();
    let signatory_reimbursement = signatory_lamports_pre.saturating_sub(signatory_lamports_post);
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

    // Debit the fee from the thread account.
    // If the worker is in the pool, pay fee to the worker's fee account.
    // Otherwise, pay fee to the worker's penalty account.
    **thread.to_account_info().try_borrow_mut_lamports()? = thread
        .to_account_info()
        .lamports()
        .checked_sub(thread.fee)
        .unwrap();
    if pool.clone().into_inner().workers.contains(&worker.key()) {
        **fee.to_account_info().try_borrow_mut_lamports()? = fee
            .to_account_info()
            .lamports()
            .checked_add(thread.fee)
            .unwrap();
    } else {
        **penalty.to_account_info().try_borrow_mut_lamports()? = penalty
            .to_account_info()
            .lamports()
            .checked_add(thread.fee)
            .unwrap();
    }

    // If the thread has no more work or the number of execs since the last payout has reached the rate limit,
    // reimburse the worker for the transaction base fee.
    if thread.next_instruction.is_none()
        || thread.exec_context.unwrap().execs_since_reimbursement >= thread.rate_limit
    {
        // Pay reimbursment for base transaction fee.
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

        // Update the exec context to mark that a reimbursement happened this slot.
        thread.exec_context = Some(ExecContext {
            execs_since_reimbursement: 0,
            ..thread.exec_context.unwrap()
        });
    }

    Ok(())
}
