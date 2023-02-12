use anchor_lang::{prelude::*, solana_program::instruction::Instruction, InstructionData};
use anchor_spl::associated_token::get_associated_token_address;
use clockwork_utils::thread::ThreadResponse;

use crate::state::*;

#[derive(Accounts)]
pub struct UnstakePreprocess<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = Registry::pubkey(),
        constraint = registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = config.epoch_thread)]
    pub thread: Signer<'info>,

    #[account(address = unstake.pubkey())]
    pub unstake: Account<'info, Unstake>,
}

pub fn handler(ctx: Context<UnstakePreprocess>) -> Result<ThreadResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let thread = &ctx.accounts.thread;
    let unstake = &ctx.accounts.unstake;

    // Return next instruction for thread.
    Ok(ThreadResponse {
        dynamic_instruction: Some(
            Instruction {
                program_id: crate::ID,
                accounts: crate::accounts::UnstakeProcess {
                    authority: unstake.authority,
                    authority_tokens: get_associated_token_address(&unstake.authority, &config.mint),
                    config: config.key(),
                    delegation: unstake.delegation,
                    registry: registry.key(),
                    thread: thread.key(),
                    token_program: anchor_spl::token::ID,
                    unstake: unstake.key(),
                    worker: unstake.worker,
                    worker_tokens: get_associated_token_address(&unstake.worker, &config.mint),
                }
                .to_account_metas(Some(true)),
                data: crate::instruction::UnstakeProcess {}.data(),
            }
            .into(),
        ),
        ..ThreadResponse::default()
    })
}
