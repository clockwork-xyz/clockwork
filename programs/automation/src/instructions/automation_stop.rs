use {crate::state::*, anchor_lang::prelude::*};

/// Accounts required by the `automation_delete` instruction.
#[derive(Accounts)]
pub struct AutomationStop<'info> {
    /// The authority (owner) of the automation.
    #[account()]
    pub authority: Signer<'info>,

    /// The automation to be paused.
    #[account(
        mut,
        seeds = [
            SEED_AUTOMATION,
            automation.authority.as_ref(),
            automation.id.as_slice(),
        ],
        bump = automation.bump,
        has_one = authority
    )]
    pub automation: Account<'info, Automation>,
}

pub fn handler(ctx: Context<AutomationStop>) -> Result<()> {
    // Get accounts
    let automation = &mut ctx.accounts.automation;

    // Pause the automation
    automation.next_instruction = None;

    Ok(())
}
