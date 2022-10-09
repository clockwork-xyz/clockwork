use {
    crate::objects::*,
    anchor_lang::{
        prelude::*,
        solana_program::system_program,
        system_program::{transfer, Transfer},
    },
};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct FeePenalize<'info> {
    #[account(
        mut,
        seeds = [
            SEED_FEE,
            fee.worker.as_ref(),
        ],
        bump,
    )]
    pub fee: Account<'info, Fee>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<FeePenalize>, amount: u64) -> Result<()> {
    // Get accounts.
    let fee = &mut ctx.accounts.fee;
    let payer = &mut ctx.accounts.payer;
    let system_program = &ctx.accounts.system_program;

    // Increment the collected fee counter.
    fee.penalty_balance = fee.penalty_balance.checked_add(amount).unwrap();

    // Transfer lamports from the payer to the fee acount.
    transfer(
        CpiContext::new(
            system_program.to_account_info(),
            Transfer {
                from: payer.to_account_info(),
                to: fee.to_account_info(),
            },
        ),
        amount,
    )?;

    Ok(())
}
