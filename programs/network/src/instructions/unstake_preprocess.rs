use anchor_spl::associated_token::get_associated_token_address;

use {
    crate::objects::*,
    anchor_lang::prelude::*,
    clockwork_utils::{anchor_sighash, AccountMetaData, CrankResponse, InstructionData},
};

#[derive(Accounts)]
pub struct UnstakePreprocess<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(address = config.epoch_queue)]
    pub queue: Signer<'info>,

    #[account(
        address = Registry::pubkey(),
        constraint = registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = unstake.pubkey())]
    pub unstake: Account<'info, Unstake>,
}

pub fn handler(ctx: Context<UnstakePreprocess>) -> Result<CrankResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let queue = &ctx.accounts.queue;
    let registry = &ctx.accounts.registry;
    let unstake = &ctx.accounts.unstake;

    // Return next instruction for queue.
    Ok(CrankResponse {
        next_instruction: Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(unstake.authority, false),
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new(unstake.delegation, false),
                AccountMetaData::new_readonly(queue.key(), true),
                AccountMetaData::new(registry.key(), false),
                AccountMetaData::new_readonly(anchor_spl::token::ID, false),
                AccountMetaData::new(unstake.key(), false),
                AccountMetaData::new_readonly(unstake.worker, false),
                AccountMetaData::new(
                    get_associated_token_address(&unstake.worker, &config.mint),
                    false,
                ),
            ],
            data: anchor_sighash("unstake_process").to_vec(),
        }),
        ..CrankResponse::default()
    })
}
