use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::{system_program, sysvar}},
    std::mem::size_of
};

#[derive(Accounts)]
#[instruction(schedule: String)]
pub struct QueueNew<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        init,
        seeds = [
            SEED_FEE, 
            queue.key().as_ref()
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Fee>(),
    )]
    pub fee: Account<'info, Fee>,

    #[account(
        mut,
        seeds = [
            SEED_MANAGER, 
            manager.authority.as_ref()
        ],
        bump,
        has_one = authority,
    )]
    pub manager: Account<'info, Manager>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_QUEUE, 
            manager.key().as_ref(),
            manager.queue_count.to_be_bytes().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Queue>(),
    )]
    pub queue: Account<'info, Queue>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<QueueNew>, schedule: String) -> Result<()> {
    let clock = &ctx.accounts.clock;
    let fee = &mut ctx.accounts.fee;
    let manager = &mut ctx.accounts.manager;
    let queue = &mut ctx.accounts.queue;

    fee.new(queue.key())?;
    queue.new(clock, manager, schedule)?;

    Ok(())
}
