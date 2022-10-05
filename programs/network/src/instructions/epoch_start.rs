use {crate::state::*, anchor_lang::prelude::*};

// This program's account structure is rooted around a trunk of Epochs.
// Epochs are iterable via their ids, auto-incrementing sequentially forward.

#[derive(Accounts)]
pub struct EpochStart<'info> {
    #[account(
        address = current_epoch.pubkey(),
        constraint = current_epoch.status.eq(EpochStatus::Current),
        has_one = snapshot,
    )]
    pub current_epoch: Account<'info, Epoch>,

    #[account(
        address = current_snapshot.pubkey(),
        constraint = current_snapshot.status.eq(SnapshotStatus::Current)
    )]
    pub current_snapshot: Account<'info, Epoch>,

    #[account(
        init,
        seeds = [
            SEED_EPOCH,
            current_epoch.id.checked_add(1).unwrap().to_be_bytes().as_ref(),
        ],
        bump,
        payer = admin,
        space = 8 + size_of::<Epoch>(),
    )]
    pub new_epoch: Account<'info, Epoch>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<EpochStart>) -> Result<()> {
    // TODO Payout yield.
    //      Transfer lamports from Fee accounts to Delegation accounts based on the current epoch's snapshot.

    // TODO Take a snapshot of Delegation stake balances.
    //      Unfreeze Delegation stake accounts and transfer CLOCK tokens from liquid epoch stake accounts to delegation stake accounts.
    //      Refreeze the delegation stake accounts.

    // TODO Capture a snapshot (cumulative sum) of the frozen stake delegation account balances.

    // TODO Mark the new epoch as "current"

    // TODO (optional) For cost-efficiency, close the snapshot accounts and return the lamports to a queue.

    Ok(())
}
