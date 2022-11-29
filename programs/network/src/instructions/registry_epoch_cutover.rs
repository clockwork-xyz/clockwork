use clockwork_utils::{anchor_sighash, AccountMetaData, InstructionData, ThreadResponse};

use {crate::state::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct RegistryEpochCutover<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [SEED_REGISTRY],
        bump,
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = config.epoch_thread)]
    pub thread: Signer<'info>,
}

pub fn handler(ctx: Context<RegistryEpochCutover>) -> Result<ThreadResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let registry = &mut ctx.accounts.registry;
    let thread = &ctx.accounts.thread;

    // Move the current epoch forward.
    registry.current_epoch = registry.current_epoch.checked_add(1).unwrap();
    registry.locked = false;

    // Build next instruction for the thread.
    // For cost-efficiency, close the prior snapshot accounts and return the lamports to the epoch thread.
    let next_instruction = Some(InstructionData {
        program_id: crate::ID,
        accounts: vec![
            AccountMetaData::new_readonly(config.key(), false),
            AccountMetaData::new_readonly(registry.key(), false),
            AccountMetaData::new(
                Snapshot::pubkey(registry.current_epoch.checked_sub(1).unwrap()),
                false,
            ),
            AccountMetaData::new(thread.key(), true),
        ],
        data: anchor_sighash("snapshot_delete").to_vec(),
    });

    Ok(ThreadResponse {
        next_instruction,
        ..ThreadResponse::default()
    })
}
