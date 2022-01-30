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
    value: Pubkey,
    bump: u8,
)]
pub struct PushElement<'info> {
    #[account(
        mut, 
        seeds = [
            SEED_LIST,
            list.owner.key().as_ref(), 
            list.namespace.as_ref()
        ],
        bump = list.bump, 
        has_one = owner,
    )]
    pub list: Account<'info, List>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_ELEMENT,
            list.key().as_ref(),
            list.count.to_be_bytes().as_ref(),
        ],
        bump = bump,
        payer = payer,
        space = 8 + size_of::<Element>(),
    )]
    pub element: Account<'info, Element>,
    
    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<PushElement>,
    value: Pubkey,
    bump: u8,
) -> ProgramResult {
    // Get accounts.
    let element = &mut ctx.accounts.element;
    let list = &mut ctx.accounts.list;
    
    // Initialize element account.
    element.index = list.count;
    element.value = value;
    element.bump = bump;

    // Increment list counter.
    list.count = list.count.checked_add(1).unwrap();
    
    return Ok(());
}
