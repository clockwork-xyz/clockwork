use clockwork_utils::CrankResponse;

use {crate::objects::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct EpochStart<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_EPOCH,
            epoch.id.to_be_bytes().as_ref(),
        ],
        bump,
        constraint = current_epoch.current
    )]
    pub current_epoch: Account<'info, Epoch>,

    #[account(
        mut,
        seeds = [
            SEED_EPOCH,
            epoch.id.to_be_bytes().as_ref(),
        ],
        bump,
        constraint = current_epoch.id.checked_add(1).unwrap().eq(&epoch.id),
    )]
    pub epoch: Account<'info, Epoch>,

    #[account(address = config.authorized_queue)]
    pub queue: Signer<'info>,
}

pub fn handler(ctx: Context<EpochStart>) -> Result<CrankResponse> {
    // Get accounts.
    let current_epoch = &mut ctx.accounts.current_epoch;
    let epoch = &mut ctx.accounts.epoch;

    // Move the current epoch forwarad
    current_epoch.current = false;
    epoch.current = true;

    Ok(CrankResponse {
        next_instruction: None,
    })
}
