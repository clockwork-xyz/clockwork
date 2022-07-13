use {
    crate::state::{Request, SEED_REQUEST},
    anchor_lang::{prelude::*, solana_program::system_program},
};

#[derive(Accounts)]
pub struct RequestAck<'info> {
    #[account(mut, address = request.ack_authority)]
    pub payer: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_REQUEST,
            request.manager.as_ref(),
            request.id.to_be_bytes().as_ref()
        ],
        bump,
    )]
    pub request: Account<'info, Request>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

// TODO: Ack data
pub fn handler<'info>(_ctx: Context<RequestAck>) -> Result<()> {
    // TODO Pay out fees to worker(s)

    Ok(())
}
