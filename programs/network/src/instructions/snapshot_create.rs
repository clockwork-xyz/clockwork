use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    anchor_spl::associated_token::get_associated_token_address,
    clockwork_utils::{anchor_sighash, AccountMetaData, InstructionData, ThreadResponse},
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct SnapshotCreate<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        address = Registry::pubkey(),
        constraint = registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT,
            registry.current_epoch.checked_add(1).unwrap().to_be_bytes().as_ref(),
        ],
        bump,
        space = 8 + size_of::<Snapshot>(),
        payer = payer
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = config.epoch_thread)]
    pub thread: Signer<'info>,
}

pub fn handler(ctx: Context<SnapshotCreate>) -> Result<ThreadResponse> {
    // Get accounts
    let config = &ctx.accounts.config;
    let registry = &ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;
    let system_program = &ctx.accounts.system_program;
    let thread = &ctx.accounts.thread;

    // Start a new snapshot.
    snapshot.init(registry.current_epoch.checked_add(1).unwrap())?;

    // Build next instruction for thread.
    let next_instruction = if registry.total_workers.gt(&0) {
        // The registry has workers. Create a snapshot frame for the zeroth worker.
        let snapshot_frame_pubkey = SnapshotFrame::pubkey(snapshot.key(), 0);
        let worker_pubkey = Worker::pubkey(0);
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new(clockwork_utils::PAYER_PUBKEY, true),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new(snapshot.key(), false),
                AccountMetaData::new(snapshot_frame_pubkey, false),
                AccountMetaData::new_readonly(system_program.key(), false),
                AccountMetaData::new_readonly(thread.key(), true),
                AccountMetaData::new_readonly(worker_pubkey, false),
                AccountMetaData::new_readonly(
                    get_associated_token_address(&worker_pubkey, &config.mint),
                    false,
                ),
            ],
            data: anchor_sighash("snapshot_frame_create").to_vec(),
        })
    } else {
        // The registry has no workers, so the snapshot is done. Start the epoch!
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new(registry.key(), false),
                AccountMetaData::new_readonly(thread.key(), true),
            ],
            data: anchor_sighash("registry_epoch_cutover").to_vec(),
        })
    };

    Ok(ThreadResponse {
        next_instruction,
        ..ThreadResponse::default()
    })
}
