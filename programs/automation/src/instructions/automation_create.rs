use std::mem::size_of;

use anchor_lang::{
    prelude::*,
    solana_program::system_program,
    system_program::{transfer, Transfer}
};
use clockwork_utils::automation::{Trigger, Ix};

use crate::state::*;

/// The minimum exec fee that may be set on an automation.
const MINIMUM_FEE: u64 = 1000;

/// Accounts required by the `automation_create` instruction.
#[derive(Accounts)]
#[instruction(amount: u64, id: Vec<u8>, instructions: Vec<Ix>,  trigger: Trigger)]
pub struct AutomationCreate<'info> {
    /// The authority (owner) of the automation.
    #[account()]
    pub authority: Signer<'info>,

    /// The payer for account initializations. 
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The Solana system program.
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    /// The automation to be created.
    #[account(
        init,
        seeds = [
            SEED_AUTOMATION,
            authority.key().as_ref(),
            id.as_slice(),
        ],
        bump,
        payer = payer,
        space = vec![
            8, 
            size_of::<Automation>(), 
            id.len(),
            instructions.try_to_vec()?.len(),  
            trigger.try_to_vec()?.len()
        ].iter().sum()
    )]
    pub automation: Account<'info, Automation>,
}

pub fn handler(ctx: Context<AutomationCreate>, amount: u64, id: Vec<u8>, instructions: Vec<Ix>, trigger: Trigger) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let payer = &ctx.accounts.payer;
    let system_program = &ctx.accounts.system_program;
    let automation = &mut ctx.accounts.automation;

    // Initialize the automation
    let bump = *ctx.bumps.get("automation").unwrap();
    automation.authority = authority.key();
    automation.bump = bump;
    automation.created_at = Clock::get().unwrap().into();
    automation.exec_context = None;
    automation.fee = MINIMUM_FEE;
    automation.id = id;
    automation.instructions = instructions;
    automation.name = String::new();
    automation.next_instruction = None;
    automation.paused = false;
    automation.rate_limit = u64::MAX;
    automation.trigger = trigger;

    // Transfer SOL from payer to the automation.
    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: payer.to_account_info(),
                to: automation.to_account_info(),
            },
        ),
        amount
    )?;

    Ok(())
}
