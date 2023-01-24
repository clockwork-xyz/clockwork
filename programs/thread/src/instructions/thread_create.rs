use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

/// The default rate limit to initialize threads with
const DEFAULT_RATE_LIMIT: u64 = 10;

/// The minimum exec fee that may be set on a thread.
const MINIMUM_FEE: u64 = 1000;

/// Accounts required by the `thread_create` instruction.
#[derive(Accounts)]
#[instruction(id: Vec<u8>, instructions: Vec<InstructionData>,  trigger: Trigger)]
pub struct ThreadCreate<'info> {
    /// The authority (owner) of the thread.
    #[account()]
    pub authority: Signer<'info>,

    /// The payer for account initializations. 
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The Solana system program.
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    /// The thread to be created.
    #[account(
        init,
        seeds = [
            SEED_THREAD,
            authority.key().as_ref(),
            id.as_slice(),
        ],
        bump,
        payer = payer,
        space = vec![
            8, 
            size_of::<Thread>(), 
            id.len(),
            instructions.try_to_vec()?.len(),  
            trigger.try_to_vec()?.len()
        ].iter().sum()
    )]
    pub thread: Account<'info, Thread>,
}

pub fn handler(ctx: Context<ThreadCreate>, id: Vec<u8>, instructions: Vec<InstructionData>, trigger: Trigger) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let thread = &mut ctx.accounts.thread;

    // Initialize the thread
    let bump = *ctx.bumps.get("thread").unwrap();
    thread.authority = authority.key();
    thread.bump = bump;
    thread.created_at = Clock::get().unwrap().into();
    thread.exec_context = None;
    thread.fee = MINIMUM_FEE;
    thread.id = id;
    thread.instructions = instructions;
    thread.name = String::new();
    thread.next_instruction = None;
    thread.paused = false;
    thread.rate_limit = DEFAULT_RATE_LIMIT;
    thread.trigger = trigger;

    Ok(())
}
