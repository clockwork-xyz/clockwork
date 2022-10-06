use {
    crate::objects::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    anchor_spl::associated_token::get_associated_token_address,
    clockwork_utils::{CrankResponse, InstructionData, anchor_sighash, AccountMetaData},
    std::mem::size_of,
};

// TODO Add condition to check if enough time has passed since the last snapshot.
//      There should be some concept of a minimum epoch duration (measured in slots).

#[derive(Accounts)]
pub struct SnapshotCreate<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = current_epoch.pubkey(),
        constraint = current_epoch.current
    )]
    pub current_epoch: Account<'info, Epoch>,

    #[account(
        address = epoch.pubkey(),
        constraint = current_epoch.id.checked_add(1).unwrap().eq(&epoch.id)
    )]
    pub epoch: Account<'info, Epoch>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = config.authorized_queue)]
    pub queue: Signer<'info>,

    #[account(
        mut, 
        seeds = [SEED_REGISTRY], 
        bump,
        constraint = !registry.locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT,
            epoch.key().as_ref(),
        ],
        bump,
        space = 8 + size_of::<Snapshot>(),
        payer = payer
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SnapshotCreate>) -> Result<CrankResponse> {
    // Get accounts
    let config = &ctx.accounts.config;
    let current_epoch = &ctx.accounts.current_epoch;
    let epoch = &ctx.accounts.epoch;
    let payer = &ctx.accounts.payer;
    let queue = &ctx.accounts.queue;
    let registry = &ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;
    let system_program = &ctx.accounts.system_program;

    // Start a new snapshot.
    snapshot.init(epoch.key())?;

    // Build next instruction for queue.
    let next_instruction = if registry.total_workers.gt(&0) {
        // The registry has workers. Create a snapshot frame for the zeroth worker.
        let snapshot_frame_pubkey = SnapshotFrame::pubkey(0, snapshot.key());
        let worker_pubkey = Worker::pubkey(0);
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(current_epoch.key(), false),
                AccountMetaData::new_readonly(epoch.key(), false),
                AccountMetaData::new(payer.key(), true),
                AccountMetaData::new_readonly(queue.key(), true),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new(snapshot.key(), false),
                AccountMetaData::new(snapshot_frame_pubkey, false),
                AccountMetaData::new_readonly(system_program.key(), false),
                AccountMetaData::new_readonly(worker_pubkey, false),
                AccountMetaData::new_readonly(get_associated_token_address(&worker_pubkey, &config.mint), false),
            ],
            data: anchor_sighash("snapshot_frame_create").to_vec()
        })
    } else {
        // The registry has no workers, so the snapshot is done. Start the epoch!
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new(current_epoch.key(), false),
                AccountMetaData::new(epoch.key(), false),
                AccountMetaData::new_readonly(queue.key(), true),
            ],
            data: anchor_sighash("epoch_start").to_vec(),
        })
    };

    Ok(CrankResponse { next_instruction })
}
