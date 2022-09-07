use {
    crate::state::*,
    anchor_lang::prelude::*,
};


#[derive(Accounts)]
pub struct QueueDelete<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub close_to: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.authority.as_ref(),
            queue.id.as_bytes(),
        ],
        bump,
        has_one = authority,
        close = close_to
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(_ctx: Context<QueueDelete>) -> Result<()> {
    Ok(())
}
