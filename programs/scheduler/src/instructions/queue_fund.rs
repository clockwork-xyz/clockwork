use {
    crate::state::*, 
    anchor_lang::{prelude::*, solana_program::system_program, system_program::{transfer, Transfer}},
};


#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct QueueFund<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.owner.as_ref()
        ],
        bump,
    )]
    pub queue: Account<'info, Queue>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<QueueFund>, amount: u64) -> Result<()> {
    let payer = &mut ctx.accounts.payer;
    let queue = &mut ctx.accounts.queue;
    let system_program = &ctx.accounts.system_program;

    transfer(
        CpiContext::new(
            system_program.to_account_info(), 
            Transfer {
                from: payer.to_account_info(),
                to: queue.to_account_info(),
            }
        ), 
        amount
    )?;

    Ok(())
}
