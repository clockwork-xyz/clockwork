use clockwork_utils::{anchor_sighash, AccountMetaData, CrankResponse, InstructionData};

use {crate::objects::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct RegistryEpochCutover<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(address = config.epoch_queue)]
    pub queue: Signer<'info>,

    #[account(
        mut,
        seeds = [SEED_REGISTRY],
        bump,
    )]
    pub registry: Account<'info, Registry>,
}

pub fn handler(ctx: Context<RegistryEpochCutover>) -> Result<CrankResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let queue = &ctx.accounts.queue;
    let registry = &mut ctx.accounts.registry;

    // Move the current epoch forward.
    registry.current_epoch = registry.current_epoch.checked_add(1).unwrap();
    registry.locked = false;

    // Build next instruction for the queue.
    // For cost-efficiency, close the prior snapshot accounts and return the lamports to the epoch queue.
    let next_instruction = Some(InstructionData {
        program_id: crate::ID,
        accounts: vec![
            AccountMetaData::new_readonly(config.key(), false),
            AccountMetaData::new(queue.key(), true),
            AccountMetaData::new_readonly(registry.key(), false),
            AccountMetaData::new(
                Snapshot::pubkey(registry.current_epoch.checked_sub(1).unwrap()),
                false,
            ),
        ],
        data: anchor_sighash("snapshot_delete").to_vec(),
    });

    Ok(CrankResponse {
        next_instruction,
        ..CrankResponse::default()
    })
}
