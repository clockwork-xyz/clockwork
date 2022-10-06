use {
    crate::objects::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    clockwork_utils::*,
    std::mem::size_of,
};

// This program's account structure is rooted around a trunk of Epochs.
// Epochs are iterable via their ids, auto-incrementing sequentially forward.

// TODO Create epoch_kickoff instruction

#[derive(Accounts)]
pub struct EpochCreate<'info> {
    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        address = current_epoch.pubkey(),
        constraint = current_epoch.current
    )]
    pub current_epoch: Account<'info, Epoch>,

    #[account(
        init,
        seeds = [
            SEED_EPOCH,
            current_epoch.id.checked_add(1).unwrap().to_be_bytes().as_ref(),
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Epoch>(),
    )]
    pub new_epoch: Account<'info, Epoch>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = config.authorized_queue)]
    pub queue: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<EpochCreate>) -> Result<CrankResponse> {
    // TODO Payout yield.
    //      Transfer collected lamports from Fee accounts to Delegation accounts based on the delegation distributions in the current Epoch's Snapshot.

    // TODO Process unstake requests.
    //      For each "unstake request" transfer tokens from the Worker stake account to the Delegation authority's token account.
    //      Decrement the Delegation's stake balance by the amount unstaked.

    // TODO Lock delegated stakes.
    //      Transfer tokens from the Delegation's stake account to the Worker's stake account.
    //      Increment the Delegation's stake balance by the amount moved.

    // TODO Take a snapshot.
    //      Capture a snapshot (cumulative sum) of the total stake and broken-down delegation balances.
    //      SnapshotFrames capture worker-level aggregate stake balances.
    //      SnapshotEntries capture delegation-level individual stake balances.

    // TODO Mark the new epoch as "current".

    // TODO (optional) For cost-efficiency, close the snapshot accounts and return the lamports to a queue.

    Ok(CrankResponse {
        next_instruction: None,
    })
}
