use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::{system_program, sysvar}},
    std::mem::size_of
};

#[derive(Accounts)]
#[instruction(
    ixs: Vec<InstructionData>,
    schedule: String,
)]
pub struct AdminQueueNew<'info> {
    #[account(mut, address = config.admin)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [SEED_AUTHORITY], 
        bump, 
    )]
    pub authority: Account<'info, Authority>,
    
    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [SEED_CONFIG],
        bump,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_YOGI,
            yogi.owner.as_ref()
        ],
        bump = yogi.bump,
        constraint = yogi.owner == authority.key(),
    )]
    pub yogi: Account<'info, Yogi>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(
        init,
        seeds = [
            SEED_QUEUE, 
            yogi.key().as_ref(),
            yogi.queue_count.to_be_bytes().as_ref(),
        ],
        bump,
        payer = admin,
        space = 8 + size_of::<Queue>() + borsh::to_vec(&ixs).unwrap().len(),
    )]
    pub queue: Account<'info, Queue>,
}

pub fn handler(
    ctx: Context<AdminQueueNew>, 
    schedule: String,
) -> Result<()> {
    let clock = &ctx.accounts.clock;
    let yogi = &mut ctx.accounts.yogi;
    let queue = &mut ctx.accounts.queue;

    queue.new(clock, yogi, schedule)
}
