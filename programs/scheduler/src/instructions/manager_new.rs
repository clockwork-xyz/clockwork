use {
    crate::state::*, 
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of
};


#[derive(Accounts)]
pub struct ManagerNew<'info> {
    #[account(
        init,
        seeds = [
            SEED_FEE, 
            manager.key().as_ref()
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Fee>(),
    )]
    pub fee: Account<'info, Fee>,

    #[account()]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_MANAGER, 
            owner.key().as_ref()
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Manager>(),
    )]
    pub manager: Account<'info, Manager>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<ManagerNew>) -> Result<()> {
    let manager = &mut ctx.accounts.manager;
    let fee = &mut ctx.accounts.fee;
    let owner = &ctx.accounts.owner;

    let manager_bump = *ctx.bumps.get("manager").unwrap();

    fee.new( manager.key())?;
    manager.new(manager_bump, owner.key())?;

    Ok(())
}
