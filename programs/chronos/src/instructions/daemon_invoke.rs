use {
    crate::state::*,
    anchor_lang::prelude::*,
    solana_program::{instruction::Instruction, program::invoke_signed},
};

#[derive(Accounts)]
#[instruction(instruction_data: InstructionData)]
pub struct DaemonInvoke<'info> {
    #[account(
        seeds = [SEED_DAEMON, daemon.owner.as_ref()],
        bump = daemon.bump,
        has_one = owner,
        owner = crate::ID
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn handler(ctx: Context<DaemonInvoke>, instruction_data: InstructionData) -> ProgramResult {
    let daemon = &ctx.accounts.daemon;

    invoke_signed(
        &Instruction::from(&instruction_data),
        &ctx.remaining_accounts.iter().as_slice(),
        &[&[SEED_DAEMON, daemon.owner.key().as_ref(), &[daemon.bump]]],
    )?;

    Ok(())
}
