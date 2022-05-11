use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(
    authority_bump: u8,
    config_bump: u8,
    fee_bump: u8,
    pool_pubkey: Pubkey,
    queue_bump: u8,
)]
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

pub fn handler(
    ctx: Context<Initialize>,
    authority_bump: u8,
    config_bump: u8,
    fee_bump: u8,
    pool_pubkey: Pubkey,
    queue_bump: u8,
) -> Result<()> {
    let admin = &ctx.accounts.admin;
    let authority = &mut ctx.accounts.authority;
    let config = &mut ctx.accounts.config;
    let queue = &mut ctx.accounts.queue;
    let fee = &mut ctx.accounts.fee;

    authority.new(authority_bump)?;
    config.new(admin.key(), config_bump, pool_pubkey)?;
    queue.new(queue_bump, authority.key())?;
    fee.new(fee_bump, queue.key())?;

    Ok(())
}
