use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
use clockwork_utils::thread::ThreadResponse;

use crate::state::*;

#[derive(Accounts)]
pub struct DeleteSnapshotJob<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = Registry::pubkey(),
        constraint = !registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = config.epoch_thread)]
    pub thread: Signer<'info>,
}

pub fn handler(ctx: Context<DeleteSnapshotJob>) -> Result<ThreadResponse> {
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let thread = &mut ctx.accounts.thread;

    Ok(ThreadResponse {
        dynamic_instruction: Some(
            Instruction {
                program_id: crate::ID,
                accounts: crate::accounts::DeleteSnapshotProcessSnapshot {
                    config: config.key(),
                    registry: registry.key(),
                    snapshot: Snapshot::pubkey(registry.current_epoch.checked_sub(1).unwrap()),
                    thread: thread.key(),
                }
                .to_account_metas(Some(true)),
                data: crate::instruction::DeleteSnapshotProcessSnapshot {}.data(),
            }
            .into(),
        ),
        close_to: None,
        trigger: None,
    })
}
