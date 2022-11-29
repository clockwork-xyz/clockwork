use {
    crate::state::*,
    anchor_lang::{
        prelude::*,
        solana_program::system_program,
        system_program::{transfer, Transfer},
    },
};

/// Accounts required by the `thread_update` instruction.
#[derive(Accounts)]
#[instruction(settings: ThreadSettings)]
pub struct ThreadUpdate<'info> {
    /// The authority (owner) of the thread.
    #[account(mut)]
    pub authority: Signer<'info>,

    /// The Solana system program
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    /// The thread to be updated.
    #[account(
            mut,
            seeds = [
                SEED_THREAD,
                thread.authority.as_ref(),
                thread.id.as_bytes(),
            ],
            bump,
            has_one = authority,
        )]
    pub thread: Account<'info, Thread>,
}

pub fn handler(ctx: Context<ThreadUpdate>, settings: ThreadSettings) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let thread = &mut ctx.accounts.thread;
    let system_program = &ctx.accounts.system_program;

    // Update the thread.
    thread.update(settings)?;

    // Reallocate mem for the thread account
    thread.realloc()?;

    // If lamports are required to maintain rent-exemption, pay them
    let data_len = 8 + thread.try_to_vec()?.len();
    let minimum_rent = Rent::get().unwrap().minimum_balance(data_len);
    if minimum_rent > thread.to_account_info().lamports() {
        transfer(
            CpiContext::new(
                system_program.to_account_info(),
                Transfer {
                    from: authority.to_account_info(),
                    to: thread.to_account_info(),
                },
            ),
            minimum_rent
                .checked_sub(thread.to_account_info().lamports())
                .unwrap(),
        )?;
    }

    Ok(())
}
