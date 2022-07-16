use {
    crate::state::*, 
    anchor_lang::{prelude::*, solana_program::system_program, system_program::{transfer, Transfer}},
};


#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct DelegateFund<'info> {
    #[account(
        mut,
        seeds = [
            SEED_DELEGATE, 
            delegate.authority.as_ref(),
        ],
        bump,
    )]
    pub delegate: Account<'info, Delegate>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DelegateFund>, amount: u64) -> Result<()> {
    let delegate = &mut ctx.accounts.delegate;
    let payer = &mut ctx.accounts.payer;
    let system_program = &ctx.accounts.system_program;

    transfer(
        CpiContext::new(
            system_program.to_account_info(), 
            Transfer {
                from: payer.to_account_info(),
                to: delegate.to_account_info(),
            }
        ), 
        amount
    )?;

    Ok(())
}
