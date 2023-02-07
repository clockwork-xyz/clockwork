use crate::{errors::ClockworkError, state::*};

use anchor_lang::{
    prelude::*,
    solana_program::system_program,
    system_program::{transfer, Transfer},
};

/// Accounts required by the `automation_update` instruction.
#[derive(Accounts)]
#[instruction(settings: AutomationSettings)]
pub struct AutomationUpdate<'info> {
    /// The authority (owner) of the automation.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The Solana system program
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    /// The automation to be updated.
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

pub fn handler(ctx: Context<AutomationUpdate>, settings: AutomationSettings) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let automation = &mut ctx.accounts.automation;
    let system_program = &ctx.accounts.system_program;

    // Update the automation.
    if let Some(fee) = settings.fee {
        automation.fee = fee;
    }

    // If provided, update the automation's instruction set.
    if let Some(instructions) = settings.instructions {
        automation.instructions = instructions;
    }

    // If provided, update the rate limit.
    if let Some(rate_limit) = settings.rate_limit {
        automation.rate_limit = rate_limit;
    }

    // If provided, update the automation's trigger and reset the exec context.
    if let Some(trigger) = settings.trigger {
        // Require the automation is not in the middle of processing.
        require!(
            std::mem::discriminant(&automation.trigger) == std::mem::discriminant(&trigger),
            ClockworkError::InvalidTriggerVariant
        );
        automation.trigger = trigger;
    }

    // Reallocate mem for the automation account
    automation.realloc()?;

    // If lamports are required to maintain rent-exemption, pay them
    let data_len = 8 + automation.try_to_vec()?.len();
    let minimum_rent = Rent::get().unwrap().minimum_balance(data_len);
    if minimum_rent > automation.to_account_info().lamports() {
        transfer(
            CpiContext::new(
                system_program.to_account_info(),
                Transfer {
                    from: authority.to_account_info(),
                    to: automation.to_account_info(),
                },
            ),
            minimum_rent
                .checked_sub(automation.to_account_info().lamports())
                .unwrap(),
        )?;
    }

    Ok(())
}
