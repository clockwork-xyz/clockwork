use anchor_lang::prelude::*;
use clockwork_macros::Clockwork;
use clockwork_utils::automation::{SerializableAccount, SerializableInstruction};

use crate::state::{Fee, FeeAccount};

#[derive(Accounts, Clockwork)]
#[instruction(amount: u64)]
pub struct FeeClaim<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub pay_to: SystemAccount<'info>,

    #[account(
        mut,
        address = fee.pubkey(),
        has_one = authority,
    )]
    pub fee: Account<'info, Fee>,
}

pub fn handler<'info>(ctx: Context<FeeClaim>, amount: u64) -> Result<()> {
    // Get accounts
    let pay_to = &mut ctx.accounts.pay_to;
    let fee = &mut ctx.accounts.fee;

    // Claim the fee funds
    fee.claim_worker_balance(amount, pay_to)?;

    // let my_ix = crate::webhook_program::fee_claim::FeeClaimInstruction::build();
    // crate::webhook_program::fee_claim::FeeClaim {
    // }

    Ok(())
}
