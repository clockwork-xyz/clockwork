use {
    crate::state::{Config, Fee, FeeAccount, Request, SEED_FEE, SEED_REQUEST},
    anchor_lang::{prelude::*, system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction()]
pub struct RequestAck<'info> {
    #[account(mut)]
    pub ack_authority: Signer<'info>,

    #[account(mut)]
    pub caller: SystemAccount<'info>,

    #[account(address = Config::pubkey())]
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
            request.caller.as_ref(),
            request.id.as_bytes(),
        ],
        bump,
        close = caller,
        has_one = caller
    )]
    pub request: Account<'info, Request>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account()]
    pub worker: SystemAccount<'info>,
}

pub fn handler<'info>(ctx: Context<RequestAck>) -> Result<()> {
    // Get accounts
    let config = &ctx.accounts.config;
    let fee = &mut ctx.accounts.fee;
    let request = &mut ctx.accounts.request;
    let worker = &mut ctx.accounts.worker;

    // Payout request fee
    let current_slot = Clock::get().unwrap().slot;
    let is_authorized_worker = request.workers.contains(&worker.key());
    let is_within_execution_window = current_slot
        < request
            .created_at
            .checked_add(config.timeout_threshold)
            .unwrap();
    if is_authorized_worker && is_within_execution_window {
        // Pay worker for executing request
        fee.pay_to_worker(request)?;
    } else {
        // Either someone is spamming or this request has timed out. Do not pay worker.
        // TODO Perhaps rather than being paid to the admin, this could be put in an escrow account where all workers could claim equal rewards.
        // TODO If not claimed within X slots, the admin can claim their rewards and close the account.
        fee.pay_to_admin(request)?;
    }

    Ok(())
}
