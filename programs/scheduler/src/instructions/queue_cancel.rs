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
            SEED_YOGI,
            yogi.owner.as_ref()
        ],
        bump,
        has_one = owner,
    )]
    pub yogi: Account<'info, Yogi>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.yogi.as_ref(),
            queue.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = yogi,
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<QueueCancel>) -> Result<()> {
    let queue = &mut ctx.accounts.queue;
    let owner = &mut ctx.accounts.owner;
    
    queue.cancel(owner)
}
