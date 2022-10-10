use {
    crate::{errors::*, objects::*},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(amount: u64, penalty: bool)]
pub struct FeeCollect<'info> {
    #[account(
        mut,
        seeds = [
            SEED_FEE,
            fee.worker.as_ref(),
        ],
        bump,
    )]
    pub fee: Account<'info, Fee>,

    #[account()]
    pub signer: Signer<'info>,
}

pub fn handler(ctx: Context<FeeCollect>, amount: u64, penalty: bool) -> Result<()> {
    // Get accounts.
    let fee = &mut ctx.accounts.fee;

    // Increment the collected fee counter.
    if penalty {
        fee.penalty_balance = fee.penalty_balance.checked_add(amount).unwrap();
    } else {
        fee.collected_balance = fee.collected_balance.checked_add(amount).unwrap();
    }

    // Verify there are enough lamports to distribute at the end of the epoch.
    let lamport_balance = fee.to_account_info().lamports();
    let data_len = 8 + fee.try_to_vec()?.len();
    let min_rent_balance = Rent::get().unwrap().minimum_balance(data_len);

    msg!(
        "Fee collection! lamports: {} collected: {} penalty: {} rent: {}",
        lamport_balance,
        fee.collected_balance,
        fee.penalty_balance,
        min_rent_balance
    );

    require!(
        fee.collected_balance
            .checked_add(fee.penalty_balance)
            .unwrap()
            .checked_add(min_rent_balance)
            .unwrap()
            .ge(&lamport_balance),
        ClockworkError::InsufficientFeeBalance
    );

    Ok(())
}
