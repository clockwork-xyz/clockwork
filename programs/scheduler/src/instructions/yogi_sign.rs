use {
    crate::state::*,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(ix: InstructionData)]
pub struct YogiSign<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_YOGI, 
            yogi.owner.as_ref()
        ],
        bump,
        has_one = owner,
    )]
    pub yogi: Account<'info, Yogi>,
}

pub fn handler(ctx: Context<YogiSign>, ix: InstructionData) -> Result<()> {
    let yogi = &mut ctx.accounts.yogi;

    let _exec_response = yogi.process(
        &ix,
        &ctx.remaining_accounts.iter().as_slice(),
    )?;

    // TODO handle exec_response

    Ok(())
}
