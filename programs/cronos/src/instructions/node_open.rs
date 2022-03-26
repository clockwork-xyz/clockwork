
use {
    crate::state::*, 
    anchor_lang::prelude::*, 
    solana_program::system_program, 
    std::mem::size_of
};

#[derive(Accounts)]
#[instruction(bump: u8, int: u128)]
pub struct NodeOpen<'info> {
    #[account(
        init,
        seeds = [
            SEED_DAEMON, 
            owner.key().as_ref(),
            int.to_be_bytes().as_ref()
        ],
        bump,
        payer = owner,
        space = 8 + size_of::<Node>(),
    )]
    pub node: Account<'info, Node>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<NodeOpen>, bump: u8, int: u128) -> Result<()> {
    let node = &mut ctx.accounts.node;
    let owner = &ctx.accounts.owner;

    node.open(bump, int, owner.key())
}
