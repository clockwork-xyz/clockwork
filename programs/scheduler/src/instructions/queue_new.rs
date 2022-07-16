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
        init_if_needed,
        seeds = [
            SEED_DELEGATE, 
            authority.key().as_ref()
        ],
        bump,
        space = 8 + size_of::<Delegate>(),
        payer = payer,
    )]
    pub delegate: Account<'info, Delegate>,

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

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_QUEUE, 
            delegate.key().as_ref(),
            delegate.queue_count.to_be_bytes().as_ref(),
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
    // Get accounts
    let authority = &ctx.accounts.authority;
    let clock = &ctx.accounts.clock;
    let delegate = &mut ctx.accounts.delegate;
    let fee = &mut ctx.accounts.fee;
    let queue = &mut ctx.accounts.queue;

    // Initialize accounts
    if !delegate.is_initialized {
        delegate.new(authority.key())?;
    }
    fee.new(queue.key())?;
    queue.new(clock, delegate, schedule)?;

    Ok(())
}
