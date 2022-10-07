use {crate::objects::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct SnapshotFrameClose<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}

pub fn handler(ctx: Context<SnapshotFrameClose>) -> Result<()> {
    Ok(())
}
