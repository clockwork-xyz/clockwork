use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    anchor_spl::associated_token::get_associated_token_address,
    clockwork_utils::{anchor_sighash, AccountMetaData, InstructionData, ThreadResponse},
};

#[derive(Accounts)]
pub struct WorkerStakeDelegations<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = Registry::pubkey(),
        constraint = registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = config.epoch_thread)]
    pub thread: Signer<'info>,

    #[account(address = worker.pubkey())]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<WorkerStakeDelegations>) -> Result<ThreadResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let thread = &ctx.accounts.thread;
    let worker = &ctx.accounts.worker;

    // Build the next instruction for the thread.
    let next_instruction = if worker.total_delegations.gt(&0) {
        // This worker has delegations. Stake their deposits.
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
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(thread.key(), true),
                AccountMetaData::new_readonly(anchor_spl::token::ID, false),
                AccountMetaData::new_readonly(worker.key(), false),
                AccountMetaData::new(
                    get_associated_token_address(&worker.key(), &config.mint),
                    false,
                ),
            ],
            data: anchor_sighash("delegation_stake").to_vec(),
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
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(thread.key(), true),
                AccountMetaData::new_readonly(
                    Worker::pubkey(worker.id.checked_add(1).unwrap()),
                    false,
                ),
            ],
            data: anchor_sighash("worker_delegations_stake").to_vec(),
        })
    } else {
        // This worker has no delegations and it is the last worker. Move on to the snapshot job!
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new(clockwork_utils::PAYER_PUBKEY, true),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new(
                    Snapshot::pubkey(registry.current_epoch.checked_add(1).unwrap()),
                    false,
                ),
                AccountMetaData::new_readonly(system_program::ID, false),
                AccountMetaData::new_readonly(thread.key(), true),
            ],
            data: anchor_sighash("snapshot_create").to_vec(),
        })
    };

    Ok(ThreadResponse {
        next_instruction,
        ..ThreadResponse::default()
    })
}
