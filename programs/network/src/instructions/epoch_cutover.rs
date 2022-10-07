use clockwork_utils::CrankResponse;

use {crate::objects::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct EpochCutover<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = epoch.pubkey(),
        constraint = registry.current_epoch_id.checked_add(1).unwrap().eq(&epoch.id),
    )]
    pub epoch: Account<'info, Epoch>,

    #[account(address = config.authorized_queue)]
    pub queue: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_REGISTRY],
        bump,
    )]
    pub registry: Account<'info, Registry>,
}

pub fn handler(ctx: Context<EpochCutover>) -> Result<CrankResponse> {
    // Get accounts.
    let epoch = &ctx.accounts.epoch;
    let registry = &mut ctx.accounts.registry;

    // Move the current epoch forward.
    registry.current_epoch_id = epoch.id;

    // TODO Build next instruction for queue.
    // TODO (optional) For cost-efficiency, close the snapshot accounts and return the lamports to a queue.

    Ok(CrankResponse {
        next_instruction: None,
    })
}
