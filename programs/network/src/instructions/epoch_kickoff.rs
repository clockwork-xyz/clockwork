use {
    crate::objects::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    clockwork_utils::{anchor_sighash, AccountMetaData, CrankResponse, InstructionData},
};

// DONE Payout yield.
//      Transfer lamports collected by Fee accounts to Delegation accounts based on the stake balance distributions of the current Epoch's SnapshotEntries.

// DONE Process unstake requests.
//      For each "unstake request" transfer tokens from the Worker stake account to the Delegation authority's token account.
//      Decrement the Delegation's stake balance by the amount unstaked.

// DONE Lock delegated stakes.
//      Transfer tokens from the Delegation's stake account to the Worker's stake account.
//      Increment the Delegation's stake balance by the amount moved.

// DONE Take a snapshot.
//      Capture a snapshot (cumulative sum) of the total stake and broken-down delegation balances.
//      SnapshotFrames capture worker-level aggregate stake balances.
//      SnapshotEntries capture delegation-level individual stake balances.

// DONE Cutover from current epoch to new epoch.

#[derive(Accounts)]
pub struct EpochKickoff<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(address = config.authorized_queue)]
    pub queue: Signer<'info>,

    #[account(address = Registry::pubkey())]
    pub registry: Account<'info, Registry>,
}

pub fn handler(ctx: Context<EpochKickoff>) -> Result<CrankResponse> {
    // Get accounts.
    let config = &ctx.accounts.config;
    let queue = &ctx.accounts.queue;
    let registry = &ctx.accounts.registry;

    // Build the next instruction for queue.
    let epoch_pubkey = Epoch::pubkey(registry.current_epoch_id.checked_add(1).unwrap());
    let next_instruction = Some(InstructionData {
        program_id: crate::ID,
        accounts: vec![
            AccountMetaData::new_readonly(config.key(), false),
            AccountMetaData::new(epoch_pubkey, false),
            AccountMetaData::new(clockwork_utils::PAYER_PUBKEY, true),
            AccountMetaData::new_readonly(queue.key(), true),
            AccountMetaData::new_readonly(registry.key(), false),
            AccountMetaData::new_readonly(system_program::ID, false),
        ],
        data: anchor_sighash("epoch_create").to_vec(),
    });

    Ok(CrankResponse { next_instruction })
}
