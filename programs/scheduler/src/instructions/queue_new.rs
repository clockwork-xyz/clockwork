use {
    crate::state::*, 
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of
};


#[derive(Accounts)]
pub struct QueueNew<'info> {
    #[account(
        init,
        seeds = [
            SEED_FEE, 
            queue.key().as_ref()
        ],
        bump,
        payer = owner,
        space = 8 + size_of::<Fee>(),
    )]
    pub fee: Account<'info, Fee>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_QUEUE, 
            owner.key().as_ref()
        ],
        bump,
        payer = owner,
        space = 8 + size_of::<Queue>(),
    )]
    pub queue: Account<'info, Queue>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<QueueNew>) -> Result<()> {
    let queue = &mut ctx.accounts.queue;
    let fee = &mut ctx.accounts.fee;
    let owner = &ctx.accounts.owner;

    let fee_bump = *ctx.bumps.get("fee").unwrap();
    let queue_bump = *ctx.bumps.get("queue").unwrap();

    fee.new(fee_bump, queue.key())?;
    queue.new(queue_bump, owner.key())
}
