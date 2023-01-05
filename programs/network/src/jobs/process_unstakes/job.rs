use {
    crate::state::*,
    anchor_lang::prelude::*,
    clockwork_utils::{anchor_sighash, AccountMetaData, InstructionData, ThreadResponse},
};

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
        next_instruction: if registry.total_unstakes.gt(&0) {
            Some(InstructionData {
                program_id: crate::ID,
                accounts: vec![
                    AccountMetaData::new_readonly(config.key(), false),
                    AccountMetaData::new_readonly(registry.key(), false),
                    AccountMetaData::new_readonly(thread.key(), true),
                    AccountMetaData::new_readonly(Unstake::pubkey(0), false),
                ],
                data: anchor_sighash("unstake_preprocess").to_vec(),
            })
        } else {
            None
        },
    })
}
