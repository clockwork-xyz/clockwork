use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};


/// Accounts required by the `thread_create` instruction.
#[derive(Accounts)]
#[instruction(id: String, kickoff_instruction: InstructionData, trigger: Trigger)]
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
            id.as_bytes(),
        ],
        bump,
        payer = payer,
        space = vec![
            8, 
            size_of::<Thread>(), 
            id.as_bytes().len(),
            kickoff_instruction.try_to_vec()?.len(),  
            trigger.try_to_vec()?.len()
        ].iter().sum()
    )]
    pub thread: Account<'info, Thread>,
}

pub fn handler(ctx: Context<ThreadCreate>, id: String, kickoff_instruction: InstructionData, trigger: Trigger) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let thread = &mut ctx.accounts.thread;

    // Initialize the thread
    thread.init(authority.key(), id, kickoff_instruction, trigger)?;

    Ok(())
}
