use {
    crate::state::*, 
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of
};


#[derive(Accounts)]
pub struct YogiNew<'info> {
    #[account(
        init,
        seeds = [
            SEED_FEE, 
            yogi.key().as_ref()
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
            SEED_YOGI, 
            owner.key().as_ref()
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<Yogi>(),
    )]
    pub yogi: Account<'info, Yogi>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<YogiNew>) -> Result<()> {
    let yogi = &mut ctx.accounts.yogi;
    let fee = &mut ctx.accounts.fee;
    let owner = &ctx.accounts.owner;

    let yogi_bump = *ctx.bumps.get("yogi").unwrap();

    fee.new( yogi.key())?;
    yogi.new(yogi_bump, owner.key())?;

    Ok(())
}
