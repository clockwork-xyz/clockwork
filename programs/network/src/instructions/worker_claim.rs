use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct WorkerClaim<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub pay_to: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [
            SEED_WORKER,
            worker.id.to_be_bytes().as_ref()
        ],
        bump,
        has_one = authority
    )]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<WorkerClaim>, amount: u64) -> Result<()> {
    // Get accounts
    let pay_to = &mut ctx.accounts.pay_to;
    let worker = &mut ctx.accounts.worker;

    // Decrement the worker's commission balance.
    worker.commission_balance = worker.commission_balance.checked_sub(amount).unwrap();

    // Transfer commission to the worker.
    **worker.to_account_info().try_borrow_mut_lamports()? = worker
        .to_account_info()
        .lamports()
        .checked_sub(amount)
        .unwrap();
    **pay_to.to_account_info().try_borrow_mut_lamports()? = pay_to
        .to_account_info()
        .lamports()
        .checked_add(amount)
        .unwrap();

    Ok(())
}
