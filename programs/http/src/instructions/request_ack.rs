use {
    crate::state::{Config, Fee, FeeAccount, Request, SEED_CONFIG, SEED_FEE, SEED_REQUEST},
    anchor_lang::{prelude::*, solana_program::sysvar, system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction()]
pub struct RequestAck<'info> {
    #[account(mut)]
    pub ack_authority: Signer<'info>,

    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(mut)]
    pub close_to: SystemAccount<'info>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Account<'info, Config>,

    #[account(
        init_if_needed,
        seeds = [
            SEED_FEE,
            worker.key().as_ref(),
        ],
        bump,
        space = 8 + size_of::<Fee>(),
        payer = ack_authority
    )]
    pub fee: Account<'info, Fee>,

    #[account(
        mut,
        seeds = [
            SEED_REQUEST,
            request.api.as_ref(),
            request.owner.as_ref(),
            request.route.as_bytes().as_ref()
        ],
        bump,
        close = close_to,
    )]
    pub request: Account<'info, Request>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account()]
    pub worker: SystemAccount<'info>,
}

pub fn handler<'info>(ctx: Context<RequestAck>) -> Result<()> {
    // Get accounts
    let clock = &ctx.accounts.clock;
    let config = &ctx.accounts.config;
    let fee = &mut ctx.accounts.fee;
    let request = &mut ctx.accounts.request;
    let worker = &mut ctx.accounts.worker;

    // Payout request fee
    let is_authorized_worker = request.workers.contains(&worker.key());
    let is_within_execution_window = clock.slot
        < request
            .created_at
            .checked_add(config.timeout_threshold)
            .unwrap();
    if is_authorized_worker && is_within_execution_window {
        // Pay worker for executing request
        fee.pay_to_worker(request)?;
    } else {
        // Either someone is spamming or this request has timed out. Do not pay worker.
        // TODO Perhaps rather than being paid to the admin, this could be put in an escrow account where all workers could claim their rewards
        // TODO If not claimed within X slots, the admin can claim their rewards and close the account
        fee.pay_to_admin(request)?;
    }

    // Payout timeout fee
    if is_within_execution_window {
        // Pay timeout fee back to the manager_authority (the account which created the request)
        // **request.to_account_info().try_borrow_mut_lamports()? = request
        //     .to_account_info()
        //     .lamports()
        //     .checked_sub(request.timeout_fee_amount)
        //     .unwrap();
        // **manager_authority
        //     .to_account_info()
        //     .try_borrow_mut_lamports()? = manager_authority
        //     .to_account_info()
        //     .lamports()
        //     .checked_add(request.timeout_fee_amount)
        //     .unwrap();
    } else {
        // Pay timeout fee into the fee account to be collected by the admin
        // fee.admin_balance = fee
        //     .admin_balance
        //     .checked_add(request.timeout_fee_amount)
        //     .unwrap();
        // **request.to_account_info().try_borrow_mut_lamports()? = request
        //     .to_account_info()
        //     .lamports()
        //     .checked_sub(request.timeout_fee_amount)
        //     .unwrap();
        // **fee.to_account_info().try_borrow_mut_lamports()? = fee
        //     .to_account_info()
        //     .lamports()
        //     .checked_add(request.timeout_fee_amount)
        //     .unwrap();
    }

    Ok(())
}
