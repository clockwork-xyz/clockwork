use {
    anchor_lang::prelude::*,
    crate::state::*,
};

#[derive(Accounts)]
pub struct PopElement<'info> {
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

    #[account(
        mut,
        seeds = [
            SEED_ELEMENT,
            list.key().as_ref(),
            element.index.to_be_bytes().as_ref(),
        ],
        bump = element.bump,
        close = owner,
        constraint = element.index == list.count - 1
    )]
    pub element: Account<'info, Element>,
}

pub fn handler(ctx: Context<PopElement>) -> ProgramResult {
    // Get accounts.
    let list = &mut ctx.accounts.list;

    // Decrement list counter.
    list.count = list.count.checked_sub(1).unwrap();
    
    return Ok(());
}
