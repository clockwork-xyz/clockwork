use {
    crate::objects::*,
    anchor_lang::{prelude::*, solana_program::system_program},
    clockwork_utils::*,
    std::mem::size_of,
};

// This program's account structure is rooted around a trunk of Epochs.
// Epochs are iterable via their ids, auto-incrementing sequentially forward.

#[derive(Accounts)]
pub struct EpochStart<'info> {
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

pub fn handler(ctx: Context<EpochStart>) -> Result<CrankResponse> {
    // TODO Payout yield.
    //      Transfer lamports from Fee accounts to Delegation accounts based on the current epoch's snapshot.

    // TODO Take a snapshot of Delegation stake balances.
    //      Unfreeze Delegation stake accounts and transfer CLOCK tokens from liquid epoch stake accounts to delegation stake accounts.
    //      Refreeze the delegation stake accounts.

    // TODO Capture a snapshot (cumulative sum) of the frozen stake delegation account balances.

    // TODO Mark the new epoch as "current"

    // TODO (optional) For cost-efficiency, close the snapshot accounts and return the lamports to a queue.

    Ok(CrankResponse { next_instruction: None })
}
