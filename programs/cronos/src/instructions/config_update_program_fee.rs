use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(
    new_program_fee: u64,
)]
pub struct ConfigUpdateProgramFee<'info> {
    #[account(
        mut,
        address = config.admin_authority,
    )]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_CONFIG],
        bump = config.bump,
        owner = crate::ID,
    )]
    pub config: Account<'info, Config>,
}

pub fn handler(ctx: Context<ConfigUpdateProgramFee>, new_program_fee: u64) -> ProgramResult {
    let config = &mut ctx.accounts.config;
    config.program_fee = new_program_fee;
    Ok(())
}
