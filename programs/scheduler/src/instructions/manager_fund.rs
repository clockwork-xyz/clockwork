use {
    crate::state::*, 
    anchor_lang::{prelude::*, solana_program::system_program, system_program::{transfer, Transfer}},
};


#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct ManagerFund<'info> {
    #[account(
        mut,
        seeds = [
            SEED_MANAGER, 
            manager.authority.as_ref(),
        ],
        bump,
    )]
    pub manager: Account<'info, Manager>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ManagerFund>, amount: u64) -> Result<()> {
    let manager = &mut ctx.accounts.manager;
    let payer = &mut ctx.accounts.payer;
    let system_program = &ctx.accounts.system_program;

    transfer(
        CpiContext::new(
            system_program.to_account_info(), 
            Transfer {
                from: payer.to_account_info(),
                to: manager.to_account_info(),
            }
        ), 
        amount
    )?;

    Ok(())
}
