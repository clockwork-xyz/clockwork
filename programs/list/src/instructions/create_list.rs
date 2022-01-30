use {
    anchor_lang::{
        prelude::*,
        solana_program::system_program
    },
    crate::state::*,
    std::mem::size_of
};

#[derive(Accounts)]
#[instruction(
    bump: u8,
)]
pub struct CreateList<'info> {
    #[account(
        init, 
        seeds = [
            SEED_LIST, 
            owner.key().as_ref(),
            namespace.key().as_ref()
        ],
        bump = bump, 
        payer = payer, 
        space = 8 + size_of::<List>()
    )]
    pub list: Account<'info, List>,

    #[account()]
    pub namespace: AccountInfo<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<CreateList>, 
    bump: u8,
) -> ProgramResult {
    // Get accounts.
    let list = &mut ctx.accounts.list;
    let namespace = &ctx.accounts.namespace;
    let owner = &ctx.accounts.owner;

    // Initialize list account.
    list.owner = owner.key();
    list.namespace = namespace.key();
    list.count = 0;
    list.bump = bump;
    
    return Ok(());
}
