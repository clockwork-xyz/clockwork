use cronos_scheduler::state::Queue;

use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct StartSnapshot<'info> {
    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Account<'info, Config>,

    #[account(signer)]
    pub queue: Account<'info, Queue>,
    // #[account(mut)]
    // pub payer: Signer<'info>,

    // #[account(
    //     init,
    //     space = 8,
    //     payer = payer
    // )]
    // pub snapshot: Account<'info, Snapshot>,

    // #[account()]
    // pub system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<StartSnapshot>) -> Result<()> {
    msg!("Starting snapshot!");
    Ok(())
}
