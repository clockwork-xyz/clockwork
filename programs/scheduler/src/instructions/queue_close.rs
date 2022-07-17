use {
    crate::state::*,
    anchor_lang::prelude::*,
};


#[derive(Accounts)]
pub struct QueueClose<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub close_to: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.authority.as_ref(),
            queue.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = authority,
        constraint = queue.status == QueueStatus::Pending || queue.status == QueueStatus::Paused,
        constraint = queue.task_count == 0,
        close = close_to
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(_ctx: Context<QueueClose>) -> Result<()> {
    Ok(())
}
