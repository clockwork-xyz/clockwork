use {
    crate::state::*,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(ix: InstructionData)]
pub struct QueueSign<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.owner.as_ref()
        ],
        bump,
        has_one = owner,
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<QueueSign>, ix: InstructionData) -> Result<()> {
    let queue = &mut ctx.accounts.queue;

    let _exec_response = queue.process(
        &ix,
        &ctx.remaining_accounts.iter().as_slice(),
    )?;

    // TODO handle exec_response

    Ok(())
}
