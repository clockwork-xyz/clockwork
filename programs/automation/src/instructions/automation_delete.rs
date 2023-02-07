use {crate::state::*, anchor_lang::prelude::*};

/// Accounts required by the `automation_delete` instruction.
#[derive(Accounts)]
pub struct AutomationDelete<'info> {
    /// The authority (owner) of the automation.
    #[account()]
    pub authority: Signer<'info>,

    /// The address to return the data rent lamports to.
    #[account(mut)]
    pub close_to: SystemAccount<'info>,

    /// The automation to be delete.
    #[account(
        mut,
        seeds = [
            SEED_AUTOMATION,
            automation.authority.as_ref(),
            automation.id.as_slice(),
        ],
        bump = automation.bump,
        has_one = authority,
        close = close_to
    )]
    pub automation: Account<'info, Automation>,
}

pub fn handler(_ctx: Context<AutomationDelete>) -> Result<()> {
    Ok(())
}
