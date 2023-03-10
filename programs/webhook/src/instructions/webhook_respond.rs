use {
    crate::state::{Webhook, SEED_WEBHOOK},
    anchor_lang::{prelude::*, system_program},
};

static TIMEOUT_THRESHOLD: u64 = 100; // 100 slots

#[derive(Accounts)]
#[instruction()]
pub struct WebhookRespond<'info> {
    #[account(mut)]
    pub ack_authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_WEBHOOK,
            webhook.authority.as_ref(),
        ],
        bump,
        // close = caller,
        // has_one = caller
    )]
    pub webhook: Account<'info, Webhook>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account()]
    pub worker: SystemAccount<'info>,
}

pub fn handler<'info>(ctx: Context<WebhookRespond>) -> Result<()> {
    // Get accounts
    // let fee = &mut ctx.accounts.fee;
    let webhook = &mut ctx.accounts.webhook;
    let worker = &mut ctx.accounts.worker;

    // Payout webhook fee
    let current_slot = Clock::get().unwrap().slot;
    let is_authorized_worker = webhook.workers.contains(&worker.key());
    let is_within_execution_window =
        current_slot < webhook.created_at.checked_add(TIMEOUT_THRESHOLD).unwrap();
    if is_authorized_worker && is_within_execution_window {
        // Pay worker for executing webhook
        // fee.pay_to_worker(webhook)?;
    } else {
        // Either someone is spamming or this webhook has timed out. Do not pay worker.
        // TODO Perhaps rather than being paid to the admin, this could be put in an escrow account where all workers could claim equal rewards.
        // TODO If not claimed within X slots, the admin can claim their rewards and close the account.
        // fee.pay_to_admin(webhook)?;
    }

    Ok(())
}
