use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
use clockwork_utils::thread::ThreadResponse;

use crate::state::*;

#[derive(Accounts)]
pub struct ProcessUnstakesJob<'info> {
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

pub fn handler(ctx: Context<ProcessUnstakesJob>) -> Result<ThreadResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let thread = &ctx.accounts.thread;

    // Return next instruction for thread.
    Ok(ThreadResponse {
        dynamic_instruction: if registry.total_unstakes.gt(&0) {
            Some(
                Instruction {
                    program_id: crate::ID,
                    accounts: crate::accounts::UnstakePreprocess {
                        config: config.key(),
                        registry: registry.key(),
                        thread: thread.key(),
                        unstake: Unstake::pubkey(0),
                    }
                    .to_account_metas(Some(true)),
                    data: crate::instruction::UnstakePreprocess {}.data(),
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
