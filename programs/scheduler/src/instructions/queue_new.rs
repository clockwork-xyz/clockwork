use {
    crate::state::*,
    anchor_lang::{prelude::*, system_program::{transfer, Transfer}, solana_program::system_program},
    std::mem::size_of
};

#[derive(Accounts)]
#[instruction(balance: u64, name: String, schedule: String)]
pub struct QueueNew<'info> {
    #[account()]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_QUEUE, 
            authority.key().as_ref(),
            name.as_bytes(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Queue>() + name.len(),
    )]
    pub queue: Account<'info, Queue>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<QueueNew>, balance: u64, name: String, schedule: String) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let payer = &mut ctx.accounts.payer;
    let queue = &mut ctx.accounts.queue;
    let system_program = &ctx.accounts.system_program;

    // Initialize accounts
    queue.new(authority.key(), name, schedule)?;

    // Transfer balance into the queue
    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: payer.to_account_info(),
                to: queue.to_account_info(),
            },
        ),
        balance,
    )?;

    Ok(())
}
