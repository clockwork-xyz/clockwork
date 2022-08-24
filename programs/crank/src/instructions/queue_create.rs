use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};


#[derive(Accounts)]
#[instruction(instruction: InstructionData, name: String, trigger: Trigger)]
pub struct QueueCreate<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_QUEUE, 
            authority.key().as_ref(),
            name.as_bytes(),
        ],
        bump,
        payer = payer,
        space = vec![
            8, 
            size_of::<Queue>(), 
            instruction.try_to_vec()?.len(), 
            name.as_bytes().len(), 
            trigger.try_to_vec()?.len()
        ].iter().sum()
    )]
    pub queue: Account<'info, Queue>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<QueueCreate>, instruction: InstructionData, name: String, trigger: Trigger) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let queue = &mut ctx.accounts.queue;

    // Initialize the queue
    queue.init(authority.key(), instruction, name, trigger)?;

    Ok(())
}
