use {
    crate::state::{Request, SEED_REQUEST},
    anchor_lang::{prelude::*, solana_program::sysvar},
};

#[derive(Accounts)]
pub struct RequestAck<'info> {
    #[account(mut)]
    pub ack_authority: Signer<'info>,

    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        mut,
        seeds = [
            SEED_REQUEST,
            request.manager.as_ref(),
            request.id.to_be_bytes().as_ref()
        ],
        bump,
        has_one = ack_authority
    )]
    pub request: Account<'info, Request>,
}

// TODO: Ack data
pub fn handler<'info>(ctx: Context<RequestAck>) -> Result<()> {
    let request = &mut ctx.accounts.request;

    msg!("Ack for request: {}", request.key());

    // TODO Pay out fees to worker(s)

    Ok(())
}
