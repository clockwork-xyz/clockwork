use {crate::state::*, anchor_lang::prelude::*, solana_program::instruction::Instruction};

#[derive(Accounts)]
#[instruction(ix: InstructionData)]
pub struct DaemonInvoke<'info> {
    #[account(
        mut,
        seeds = [SEED_DAEMON, daemon.owner.as_ref()],
        bump = daemon.bump,
        has_one = owner,
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn handler(ctx: Context<DaemonInvoke>, ix: InstructionData) -> Result<()> {
    let daemon = &mut ctx.accounts.daemon;

    daemon.invoke(
        &Instruction::from(&ix),
        &ctx.remaining_accounts.iter().as_slice(),
    )
}
