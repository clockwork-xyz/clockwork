use {
    anchor_lang::prelude::*,
    crate::state::*,
};

#[derive(Accounts)]
pub struct DeleteList<'info> {
    #[account(
        mut, 
        seeds = [
            SEED_LIST, 
            list.owner.key().as_ref(), 
            list.namespace.as_ref()
        ],
        bump = list.bump, 
        has_one = owner,
        close = owner,
        constraint = list.count == 0
    )]
    pub list: Account<'info, List>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

pub fn handler(_ctx: Context<DeleteList>) -> ProgramResult {
    return Ok(());
}
