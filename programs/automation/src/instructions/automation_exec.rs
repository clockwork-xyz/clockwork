use anchor_lang::{
    prelude::*,
    solana_program::{
        instruction::Instruction,
        program::{get_return_data, invoke_signed},
    },
    AnchorDeserialize,
};
use clockwork_network_program::state::{Fee, Pool, Worker, WorkerAccount};
use clockwork_utils::automation::{AutomationResponse, PAYER_PUBKEY};

use crate::{errors::ClockworkError, state::*};

/// The ID of the pool workers must be a member of to collect fees.
const POOL_ID: u64 = 0;

/// The number of lamports to reimburse the worker with after they've submitted a transaction's worth of exec instructions.
const TRANSACTION_BASE_FEE_REIMBURSEMENT: u64 = 5_000;

/// Accounts required by the `automation_exec` instruction.
#[derive(Accounts)]
pub struct AutomationExec<'info> {
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

    /// The automation to execute.
    #[account(
        mut,
        seeds = [
            SEED_AUTOMATION,
            automation.authority.as_ref(),
            automation.id.as_slice(),
        ],
        bump = automation.bump,
        constraint = !automation.paused @ ClockworkError::AutomationPaused,
        constraint = automation.next_instruction.is_some(),
        constraint = automation.exec_context.is_some()
    )]
    pub automation: Box<Account<'info, Automation>>,

    /// The worker.
    #[account(address = worker.pubkey())]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<AutomationExec>) -> Result<()> {
    // Get accounts
    let fee = &mut ctx.accounts.fee;
    let pool = &ctx.accounts.pool;
    let signatory = &mut ctx.accounts.signatory;
    let automation = &mut ctx.accounts.automation;
    let worker = &ctx.accounts.worker;

    // If the rate limit has been met, exit early.
    if automation.exec_context.unwrap().last_exec_at == Clock::get().unwrap().slot
        && automation.exec_context.unwrap().execs_since_slot >= automation.rate_limit
    {
        return Err(ClockworkError::RateLimitExeceeded.into());
    }

    // Record the worker's lamports before invoking inner ixs.
    let signatory_lamports_pre = signatory.lamports();

    // Get the instruction to execute.
    // We have already verified that it is not null during account validation.
    let next_instruction: &Option<SerializableInstruction> = &automation.clone().next_instruction;
    let instruction = next_instruction.as_ref().unwrap();

    // Inject the signatory's pubkey for the Clockwork payer ID.
    let normalized_accounts: &mut Vec<AccountMeta> = &mut vec![];
    instruction.accounts.iter().for_each(|acc| {
        let acc_pubkey = if acc.pubkey == PAYER_PUBKEY {
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
            SEED_AUTOMATION,
            automation.authority.as_ref(),
            automation.id.as_slice(),
            &[automation.bump],
        ]],
    )?;

    // Verify the inner instruction did not write data to the signatory address.
    require!(signatory.data_is_empty(), ClockworkError::UnauthorizedWrite);

    // Parse the automation response
    let automation_response: Option<AutomationResponse> = match get_return_data() {
        None => None,
        Some((program_id, return_data)) => {
            require!(
                program_id.eq(&instruction.program_id),
                ClockworkError::InvalidAutomationResponse
            );
            AutomationResponse::try_from_slice(return_data.as_slice()).ok()
        }
    };

    // Grab the next instruction from the automation response.
    let mut next_instruction = None;
    if let Some(automation_response) = automation_response {
        next_instruction = automation_response.dynamic_instruction;

        // Update the trigger.
        if let Some(trigger) = automation_response.trigger {
            require!(
                std::mem::discriminant(&automation.trigger) == std::mem::discriminant(&trigger),
                ClockworkError::InvalidTriggerVariant
            );
            automation.trigger = trigger;
        }
    }

    // If there is no dynamic next instruction, get the next instruction from the instruction set.
    let mut exec_index = automation.exec_context.unwrap().exec_index;
    if next_instruction.is_none() {
        if let Some(ix) = automation.instructions.get((exec_index + 1) as usize) {
            next_instruction = Some(ix.clone());
            exec_index = exec_index + 1;
        }
    }

    // Update the next instruction.
    automation.next_instruction = next_instruction;

    // Update the exec context.
    let current_slot = Clock::get().unwrap().slot;
    automation.exec_context = Some(ExecContext {
        exec_index,
        execs_since_reimbursement: automation
            .exec_context
            .unwrap()
            .execs_since_reimbursement
            .checked_add(1)
            .unwrap(),
        execs_since_slot: if current_slot == automation.exec_context.unwrap().last_exec_at {
            automation
                .exec_context
                .unwrap()
                .execs_since_slot
                .checked_add(1)
                .unwrap()
        } else {
            1
        },
        last_exec_at: current_slot,
        ..automation.exec_context.unwrap()
    });

    // Realloc memory for the automation account.
    automation.realloc()?;

    // Reimbursement signatory for lamports paid during inner ix.
    let signatory_lamports_post = signatory.lamports();
    let signatory_reimbursement = signatory_lamports_pre.saturating_sub(signatory_lamports_post);
    if signatory_reimbursement.gt(&0) {
        **automation.to_account_info().try_borrow_mut_lamports()? = automation
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

    // If the worker is in the pool, debit from the automation account and payout to the worker's fee account.
    if pool.clone().into_inner().workers.contains(&worker.key()) {
        **automation.to_account_info().try_borrow_mut_lamports()? = automation
            .to_account_info()
            .lamports()
            .checked_sub(automation.fee)
            .unwrap();
        **fee.to_account_info().try_borrow_mut_lamports()? = fee
            .to_account_info()
            .lamports()
            .checked_add(automation.fee)
            .unwrap();
    }

    // If the automation has no more work or the number of execs since the last payout has reached the rate limit,
    // reimburse the worker for the transaction base fee.
    if automation.next_instruction.is_none()
        || automation.exec_context.unwrap().execs_since_reimbursement >= automation.rate_limit
    {
        // Pay reimbursment for base transaction fee.
        **automation.to_account_info().try_borrow_mut_lamports()? = automation
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
        automation.exec_context = Some(ExecContext {
            execs_since_reimbursement: 0,
            ..automation.exec_context.unwrap()
        });
    }

    Ok(())
}
