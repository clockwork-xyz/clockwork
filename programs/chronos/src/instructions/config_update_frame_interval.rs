use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
#[instruction(
    new_frame_interval: u64,
)]
pub struct ConfigUpdateFrameInterval<'info> {
    #[account(
        mut,
        seeds = [SEED_CONFIG],
        bump = config.bump,
        owner = crate::ID,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        address = config.admin_authority,
    )]
    pub signer: Signer<'info>,
}

pub fn handler(ctx: Context<ConfigUpdateFrameInterval>, new_frame_interval: u64) -> ProgramResult {
    let config = &mut ctx.accounts.config;
    config.frame_interval = new_frame_interval;
    Ok(())
}
