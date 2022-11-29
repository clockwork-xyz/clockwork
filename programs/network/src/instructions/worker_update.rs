use {
    crate::state::*,
    anchor_lang::{
        prelude::*,
        solana_program::system_program,
        system_program::{transfer, Transfer},
    },
};

#[derive(Accounts)]
#[instruction(settings: WorkerSettings)]
pub struct WorkerUpdate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        seeds = [
            SEED_WORKER,
            worker.id.to_be_bytes().as_ref(),
        ],
        bump,
        has_one = authority,
    )]
    pub worker: Account<'info, Worker>,
}

pub fn handler(ctx: Context<WorkerUpdate>, settings: WorkerSettings) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let worker = &mut ctx.accounts.worker;
    let system_program = &ctx.accounts.system_program;

    // Update the worker
    worker.update(settings)?;

    // Realloc memory for the worker account
    let data_len = 8 + worker.try_to_vec()?.len();
    worker.to_account_info().realloc(data_len, false)?;

    // If lamports are required to maintain rent-exemption, pay them
    let minimum_rent = Rent::get().unwrap().minimum_balance(data_len);
    if minimum_rent > worker.to_account_info().lamports() {
        transfer(
            CpiContext::new(
                system_program.to_account_info(),
                Transfer {
                    from: authority.to_account_info(),
                    to: worker.to_account_info(),
                },
            ),
            minimum_rent
                .checked_sub(worker.to_account_info().lamports())
                .unwrap(),
        )?;
    }

    Ok(())
}
