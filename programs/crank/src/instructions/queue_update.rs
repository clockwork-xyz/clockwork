use {
    crate::state::*,
    anchor_lang::prelude::*,
};


#[derive(Accounts)]
#[instruction(first_instruction: Option<InstructionData>, trigger: Option<Trigger>)]
pub struct QueueUpdate<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.authority.key().as_ref(),
            queue.name.as_bytes(),
        ],
        bump,
        has_one = authority,
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<QueueUpdate>, first_instruction: Option<InstructionData>, trigger: Option<Trigger>) -> Result<()> {
    // Get accounts
    let queue = &mut ctx.accounts.queue;

    // If provided, update the queue's first instruction
    if let Some(first_instruction) = first_instruction {
        queue.first_instruction = first_instruction;
    }

    // If provided, update the queue's trigger and reset the exec context
    if let Some(trigger) = trigger {
        queue.trigger = trigger;
        queue.exec_context = None;
    }

    // Reallocate mem for the queue account
    queue.realloc()?;

    Ok(())
}