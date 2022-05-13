use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::instruction::Instruction},
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

    queue.sign(
        &Instruction::from(&ix),
        &ctx.remaining_accounts.iter().as_slice(),
    )?;

    Ok(())
}
