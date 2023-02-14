use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
use clockwork_utils::thread::ThreadResponse;

use crate::state::*;

#[derive(Accounts)]
pub struct StakeDelegationsJob<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = Registry::pubkey(),
        constraint = registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = config.epoch_thread)]
    pub thread: Signer<'info>,
}

pub fn handler(ctx: Context<StakeDelegationsJob>) -> Result<ThreadResponse> {
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let thread = &ctx.accounts.thread;

    Ok(ThreadResponse {
        dynamic_instruction: if registry.total_workers.gt(&0) {
            Some(
                Instruction {
                    program_id: crate::ID,
                    accounts: crate::accounts::StakeDelegationsProcessWorker {
                        config: config.key(),
                        registry: registry.key(),
                        thread: thread.key(),
                        worker: Worker::pubkey(0),
                    }
                    .to_account_metas(Some(true)),
                    data: crate::instruction::StakeDelegationsProcessWorker {}.data(),
                }
                .into(),
            )
        } else {
            None
        },
        close_to: None,
        trigger: None,
    })
}
