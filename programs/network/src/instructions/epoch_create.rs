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

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<EpochCreate>) -> Result<CrankResponse> {
    Ok(CrankResponse {
        next_instruction: None,
    })
}
