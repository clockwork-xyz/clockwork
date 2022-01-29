use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(bump: u8)]
pub struct AgentCreate<'info> {
    #[account(
        init,
        seeds = [SEED_AGENT, signer.key().as_ref()],
        bump = bump,
        payer = signer,
        space = 8 + size_of::<Agent>(),
    )]
    pub agent: Account<'info, Agent>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<AgentCreate>, bump: u8) -> ProgramResult {
    // Get accounts.
    let agent = &mut ctx.accounts.agent;
    let signer = &ctx.accounts.signer;

    // Initialize agent account.
    agent.owner = signer.key();
    agent.bump = bump;

    return Ok(());
}
