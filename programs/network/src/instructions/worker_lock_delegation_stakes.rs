use {
    crate::objects::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    anchor_spl::{
        associated_token::get_associated_token_address,
        token::{transfer, Mint, Token, TokenAccount, Transfer},
    },
    clockwork_utils::{anchor_sighash, AccountMetaData, CrankResponse, InstructionData},
};

#[derive(Accounts)]
pub struct WorkerLockDelegationStakes<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(address = config.authorized_queue)]
    pub queue: Signer<'info>,

    #[account(
        address = Registry::pubkey(),
        constraint = registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(
        mut,
        seeds = [
            SEED_WORKER,
            worker.id.to_be_bytes().as_ref()
        ],
        bump,
    )]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<WorkerLockDelegationStakes>) -> Result<CrankResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let queue = &ctx.accounts.queue;
    let registry = &ctx.accounts.registry;
    let worker = &ctx.accounts.worker;

    // Build the next instruction for the queue.
    let next_instruction = if worker.total_delegations.gt(&0) {
        // This worker has delegations. Lock their stakes.
        let delegation_pubkey = Delegation::pubkey(worker.key(), 0);
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new(delegation_pubkey, false),
                AccountMetaData::new(
                    get_associated_token_address(&delegation_pubkey, &config.mint),
                    false,
                ),
                AccountMetaData::new_readonly(queue.key(), true),
                AccountMetaData::new_readonly(Registry::pubkey(), false),
                AccountMetaData::new_readonly(anchor_spl::token::ID, false),
                AccountMetaData::new_readonly(worker.key(), false),
                AccountMetaData::new(
                    get_associated_token_address(&worker.key(), &config.mint),
                    false,
                ),
            ],
            data: anchor_sighash("delegation_lock_stake").to_vec(),
        })
    } else if worker
        .id
        .checked_add(1)
        .unwrap()
        .lt(&registry.total_workers)
    {
        // This worker has no delegations. Move on to the next worker.
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(queue.key(), true),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(worker.key(), false),
            ],
            data: anchor_sighash("worker_lock_delegation_stakes").to_vec(),
        })
    } else {
        // TODO This must be the last worker. Move on to the snapshot job!
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(queue.key(), true),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(worker.key(), false),
            ],
            data: anchor_sighash("snapshot_create").to_vec(),
        })
    };

    Ok(CrankResponse { next_instruction })
}
