use {
    crate::{errors::*, state::*},
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
pub struct PenaltyClaim<'info> {
    #[account(
        mut, 
        address = config.admin,
    )]
    pub admin: Signer<'info>,

    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub pay_to: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [
            SEED_PENALTY,
            penalty.worker.as_ref(),
        ],
        bump,
    )]
    pub penalty: Account<'info, Penalty>,
}

pub fn handler(ctx: Context<PenaltyClaim>) -> Result<()> {
    // Get accounts.
    let penalty = &mut ctx.accounts.penalty;
    let pay_to = &mut ctx.accounts.pay_to;
 
    // Calculate how  many lamports are 
    let lamport_balance = penalty.to_account_info().lamports();
    let data_len = 8 + penalty.try_to_vec()?.len();
    let min_rent_balance = Rent::get().unwrap().minimum_balance(data_len);
    let claimable_balance = lamport_balance.checked_sub(min_rent_balance).unwrap();
    require!(claimable_balance.gt(&0), ClockworkError::InsufficientPenaltyBalance);

    // Pay reimbursment for base transaction fee
    **penalty.to_account_info().try_borrow_mut_lamports()? = penalty
        .to_account_info()
        .lamports()
        .checked_sub(claimable_balance)
        .unwrap();
    **pay_to.to_account_info().try_borrow_mut_lamports()? = pay_to
        .to_account_info()
        .lamports()
        .checked_add(claimable_balance)
        .unwrap();

    Ok(())
}
