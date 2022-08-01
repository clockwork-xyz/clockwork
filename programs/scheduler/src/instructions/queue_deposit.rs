

use {
    crate::state::*,
    anchor_lang::{prelude::*, system_program::{transfer, Transfer}, solana_program::system_program},
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct QueueDeposit<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.authority.key().as_ref(),
            queue.name.as_bytes(),
        ],
        bump,
    )]
    pub queue: Account<'info, Queue>,

    #[account()]
    pub signer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<QueueDeposit>, amount: u64) -> Result<()> {
    // Get accounts
    let payer = &mut ctx.accounts.payer;
    let queue = &mut ctx.accounts.queue;
    let system_program = &ctx.accounts.system_program;

    // Transfer balance into the queue
    queue.balance = queue.balance.checked_add(amount).unwrap();
    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: payer.to_account_info(),
                to: queue.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}
