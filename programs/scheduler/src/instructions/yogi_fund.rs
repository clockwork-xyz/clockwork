use {
    crate::state::*, 
    anchor_lang::{prelude::*, solana_program::system_program, system_program::{transfer, Transfer}},
};


#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct YogiFund<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_YOGI, 
            yogi.owner.as_ref()
        ],
        bump,
    )]
    pub yogi: Account<'info, Yogi>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<YogiFund>, amount: u64) -> Result<()> {
    let payer = &mut ctx.accounts.payer;
    let yogi = &mut ctx.accounts.yogi;
    let system_program = &ctx.accounts.system_program;

    transfer(
        CpiContext::new(
            system_program.to_account_info(), 
            Transfer {
                from: payer.to_account_info(),
                to: yogi.to_account_info(),
            }
        ), 
        amount
    )?;

    Ok(())
}
