use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(pool_pubkey: Pubkey)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [SEED_AUTHORITY],
        bump,
        payer = admin,
        space = 8 + size_of::<Authority>(),
    )]
    pub authority: Account<'info, Authority>,

    #[account(
        init,
        seeds = [SEED_CONFIG],
        bump,
        payer = admin,
        space = 8 + size_of::<Config>(),
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        seeds = [
            SEED_FEE, 
            queue.key().as_ref()
        ],
        bump,
        payer = admin,
        space = 8 + size_of::<Fee>(),
    )]
    pub fee: Account<'info, Fee>,

    #[account(
        init,
        seeds = [
            SEED_QUEUE,
            authority.key().as_ref()
        ],
        bump,
        payer = admin,
        space = 8 + size_of::<Queue>(),
    )]
    pub queue: Account<'info, Queue>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<Initialize>, pool_pubkey: Pubkey) -> Result<()> {
    let admin = &ctx.accounts.admin;
    let authority = &mut ctx.accounts.authority;
    let config = &mut ctx.accounts.config;
    let queue = &mut ctx.accounts.queue;
    let fee = &mut ctx.accounts.fee;

    let queue_bump = *ctx.bumps.get("queue").unwrap();

    config.new(admin.key(), pool_pubkey)?;
    queue.new(queue_bump, authority.key())?;
    fee.new(queue.key())?;

    Ok(())
}
