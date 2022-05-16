use {
    crate::state::*,
    anchor_lang::prelude::*
};

#[derive(Accounts)]
#[instruction()]
pub struct AdminQueueCancel<'info> {
    #[account(
        mut, 
        address = config.admin
    )]
    pub admin: Signer<'info>,

    #[account(
        seeds = [SEED_CONFIG],
        bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.yogi.as_ref(),
            queue.id.to_be_bytes().as_ref(),
        ],
        bump,
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<AdminQueueCancel>) -> Result<()> {
    let admin = &mut ctx.accounts.admin;
    let queue = &mut ctx.accounts.queue;
    
    queue.cancel(admin)
}
