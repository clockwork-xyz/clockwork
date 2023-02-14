use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
use clockwork_utils::thread::ThreadResponse;

use crate::state::*;

#[derive(Accounts)]
pub struct DistributeFeesJob<'info> {
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

pub fn handler(ctx: Context<DistributeFeesJob>) -> Result<ThreadResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let registry = &mut ctx.accounts.registry;
    let thread = &ctx.accounts.thread;

    // Lock the registry.
    registry.locked = true;

    // Process the snapshot.
    Ok(ThreadResponse {
        dynamic_instruction: Some(
            Instruction {
                program_id: crate::ID,
                accounts: crate::accounts::DistributeFeesProcessSnapshot {
                    config: config.key(),
                    registry: registry.key(),
                    snapshot: Snapshot::pubkey(registry.current_epoch),
                    thread: thread.key(),
                }
                .to_account_metas(Some(true)),
                data: crate::instruction::DistributeFeesProcessSnapshot {}.data(),
            }
            .into(),
        ),
        close_to: None,
        trigger: None,
    })
}
