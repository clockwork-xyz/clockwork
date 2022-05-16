use {
    crate::state::*,
    anchor_lang::prelude::*,
};

#[derive(Accounts)]
#[instruction(ix: InstructionData)]
pub struct ManagerSign<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_MANAGER, 
            manager.owner.as_ref()
        ],
        bump,
        has_one = owner,
    )]
    pub manager: Account<'info, Manager>,
}

pub fn handler(ctx: Context<ManagerSign>, ix: InstructionData) -> Result<()> {
    let manager = &mut ctx.accounts.manager;

    let _exec_response = manager.process(
        &ix,
        &ctx.remaining_accounts.iter().as_slice(),
    )?;

    // TODO handle exec_response

    Ok(())
}
