use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::{system_program, sysvar}},
    std::mem::size_of
};

#[derive(Accounts)]
#[instruction(schedule: String)]
pub struct QueueNew<'info> {
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account()]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_MANAGER, 
            manager.owner.as_ref()
        ],
        bump = manager.bump,
        has_one = owner,
    )]
    pub manager: Account<'info, Manager>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(
        init,
        seeds = [
            SEED_QUEUE, 
            manager.key().as_ref(),
            manager.queue_count.to_be_bytes().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Queue>(), // + borsh::to_vec(&ixs).unwrap().len(),
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<QueueNew>, schedule: String) -> Result<()> {
    let clock = &ctx.accounts.clock;
    let manager = &mut ctx.accounts.manager;
    let queue = &mut ctx.accounts.queue;

    queue.new(clock, manager, schedule)?;

    Ok(())
}
