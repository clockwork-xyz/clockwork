use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    solana_program::{instruction::Instruction, program::invoke_signed},
};

#[derive(Accounts)]
#[instruction(instruction_data: InstructionData)]
pub struct DaemonInvoke<'info> {
    #[account(
        seeds = [SEED_DAEMON, signer.key().as_ref()],
        bump = daemon.bump,
        constraint = daemon.owner == signer.key(),
        owner = crate::ID
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<DaemonInvoke>, instruction_data: InstructionData) -> ProgramResult {
    let daemon = &ctx.accounts.daemon;
    invoke_signed(
        &Instruction::from(&instruction_data),
        &mut ctx.remaining_accounts.iter().as_slice(),
        &[&[SEED_DAEMON, daemon.owner.key().as_ref(), &[daemon.bump]]],
    )?;
    return Ok(());
}
