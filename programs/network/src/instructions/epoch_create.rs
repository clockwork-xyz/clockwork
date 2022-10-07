use {
    crate::objects::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    clockwork_utils::*,
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct EpochCreate<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        init,
        seeds = [
            SEED_EPOCH,
            registry.current_epoch_id.checked_add(1).unwrap().to_be_bytes().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Epoch>(),
    )]
    pub epoch: Account<'info, Epoch>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = config.authorized_queue)]
    pub queue: Signer<'info>,

    #[account(address = Registry::pubkey())]
    pub registry: Account<'info, Registry>,

    #[account(
        address = snapshot.pubkey(),
        has_one = epoch,
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<EpochCreate>) -> Result<CrankResponse> {
    // Get accounts
    let config = &ctx.accounts.config;
    let epoch = &mut ctx.accounts.epoch;
    let queue = &ctx.accounts.queue;
    let registry = &ctx.accounts.registry;
    let snapshot = &ctx.accounts.snapshot;

    // Initialize the next epoch account.
    epoch.init(registry.current_epoch_id.checked_add(1).unwrap())?;

    // Build next instruction for the queue.
    let next_instruction = if snapshot.total_frames.gt(&0) {
        // The current snapshot has frames. Distribute fees collected by workers.
        let worker_pubkey = Worker::pubkey(0);
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(epoch.key(), false),
                AccountMetaData::new(Fee::pubkey(worker_pubkey), false),
                AccountMetaData::new_readonly(queue.key(), true),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(snapshot.key(), false),
                AccountMetaData::new_readonly(SnapshotFrame::pubkey(0, snapshot.key()), false),
                AccountMetaData::new_readonly(worker_pubkey, false),
            ],
            data: anchor_sighash("worker_distribute_fees").to_vec(),
        })
    } else if registry.total_workers.gt(&0) {
        // The registry has workers. Begin staking delegations.
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(queue.key(), true),
                AccountMetaData::new_readonly(registry.key(), false),
                AccountMetaData::new_readonly(Worker::pubkey(0), false),
            ],
            data: anchor_sighash("worker_stake_delegations").to_vec(),
        })
    } else {
        // Cutover from current epoch to new.
        Some(InstructionData {
            program_id: crate::ID,
            accounts: vec![
                AccountMetaData::new_readonly(config.key(), false),
                AccountMetaData::new_readonly(epoch.key(), false),
                AccountMetaData::new_readonly(queue.key(), true),
                AccountMetaData::new(registry.key(), false),
            ],
            data: anchor_sighash("epoch_cutover").to_vec(),
        })
    };

    Ok(CrankResponse { next_instruction })
}
