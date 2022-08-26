use {crate::state::*, anchor_lang::prelude::*, clockwork_pool::state::Pool};

#[derive(Accounts)]
pub struct NodeDropPool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_NODE,
            node.id.to_be_bytes().as_ref()
        ],
        bump,
        has_one = authority,
    )]
    pub node: Account<'info, Node>,

    #[account(owner = clockwork_pool::ID)]
    pub pool: Account<'info, Pool>,
}

pub fn handler(ctx: Context<NodeDropPool>) -> Result<()> {
    // Get accounts
    let node = &mut ctx.accounts.node;
    let pool = &mut ctx.accounts.pool;

    // Update the pool
    node.drop_pool(pool.key())?;

    Ok(())
}
