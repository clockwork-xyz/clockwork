use {
    crate::objects::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};


/// Accounts required by the `queue_create` instruction.
#[derive(Accounts)]
#[instruction(id: String, kickoff_instruction: clockwork_utils::InstructionData, trigger: Trigger)]
pub struct QueueCreate<'info> {
    /// The authority (owner) of the queue.
    #[account()]
    pub authority: Signer<'info>,

    /// The payer for account initializations. 
    #[account(mut)]
    pub payer: Signer<'info>,

    /// The queue to be created.
    #[account(
        init,
        seeds = [
            SEED_QUEUE,
            authority.key().as_ref(),
            id.as_bytes(),
        ],
        bump,
        payer = payer,
        space = vec![
            8, 
            size_of::<Queue>(), 
            id.as_bytes().len(),
            kickoff_instruction.try_to_vec()?.len(),  
            trigger.try_to_vec()?.len()
        ].iter().sum()
    )]
    pub queue: Account<'info, Queue>,

    /// The Solana system program.
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<QueueCreate>, id: String, kickoff_instruction: clockwork_utils::InstructionData, trigger: Trigger) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let queue = &mut ctx.accounts.queue;

    // Initialize the queue
    queue.init(authority.key(), id, kickoff_instruction, trigger)?;

    Ok(())
}
