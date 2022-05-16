use {
    crate::state::*,
    anchor_lang::prelude::*
};

#[derive(Accounts)]
#[instruction()]
pub struct QueueCancel<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        seeds = [
            SEED_MANAGER,
            manager.owner.as_ref()
        ],
        bump,
        has_one = owner,
    )]
    pub manager: Account<'info, Manager>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.manager.as_ref(),
            queue.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = manager,
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<QueueCancel>) -> Result<()> {
    let queue = &mut ctx.accounts.queue;
    let owner = &mut ctx.accounts.owner;
    
    queue.cancel(owner)
}
