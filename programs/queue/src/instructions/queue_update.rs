use {
    crate::objects::*,
    anchor_lang::{
        prelude::*,
        solana_program::system_program,
        system_program::{transfer, Transfer},
    },
};

/// Accounts required by the `queue_update` instruction.
#[derive(Accounts)]
#[instruction(settings: QueueSettings)]
pub struct QueueUpdate<'info> {
    /// The authority (owner) of the queue.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The queue to be updated.
    #[account(
        mut,
        seeds = [
            SEED_QUEUE,
            queue.authority.as_ref(),
            queue.id.as_bytes(),
        ],
        bump,
        has_one = authority,
    )]
    pub queue: Account<'info, Queue>,

    /// The Solana system program
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<QueueUpdate>, settings: QueueSettings) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let queue = &mut ctx.accounts.queue;
    let system_program = &ctx.accounts.system_program;

    // Update the queue.
    queue.update(settings)?;

    // Reallocate mem for the queue account
    queue.realloc()?;

    // If lamports are required to maintain rent-exemption, pay them
    let data_len = 8 + queue.try_to_vec()?.len();
    let minimum_rent = Rent::get().unwrap().minimum_balance(data_len);
    if minimum_rent > queue.to_account_info().lamports() {
        transfer(
            CpiContext::new(
                system_program.to_account_info(),
                Transfer {
                    from: authority.to_account_info(),
                    to: queue.to_account_info(),
                },
            ),
            minimum_rent
                .checked_sub(queue.to_account_info().lamports())
                .unwrap(),
        )?;
    }

    Ok(())
}
