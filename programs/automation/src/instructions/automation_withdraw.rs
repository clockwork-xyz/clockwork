use {
    crate::{errors::*, state::*},
    anchor_lang::prelude::*,
};

/// Accounts required by the `automation_withdraw` instruction.
#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct AutomationWithdraw<'info> {
    /// The authority (owner) of the automation.
    #[account()]
    pub authority: Signer<'info>,

    /// The account to withdraw lamports to.
    #[account(mut)]
    pub pay_to: SystemAccount<'info>,

    /// The automation to be.
    #[account(
        mut,
        seeds = [
            SEED_AUTOMATION,
            automation.authority.as_ref(),
            automation.id.as_slice(),
        ],
        bump = automation.bump,
        has_one = authority,
    )]
    pub automation: Account<'info, Automation>,
}

pub fn handler(ctx: Context<AutomationWithdraw>, amount: u64) -> Result<()> {
    // Get accounts
    let pay_to = &mut ctx.accounts.pay_to;
    let automation = &mut ctx.accounts.automation;

    // Calculate the minimum rent threshold
    let data_len = 8 + automation.try_to_vec()?.len();
    let minimum_rent = Rent::get().unwrap().minimum_balance(data_len);
    let post_balance = automation
        .to_account_info()
        .lamports()
        .checked_sub(amount)
        .unwrap();
    require!(
        post_balance.gt(&minimum_rent),
        ClockworkError::WithdrawalTooLarge
    );

    // Withdraw balance from automation to the pay_to account
    **automation.to_account_info().try_borrow_mut_lamports()? = automation
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
