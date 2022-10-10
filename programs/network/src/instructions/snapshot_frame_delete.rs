use {crate::objects::*, anchor_lang::prelude::*};

#[derive(Accounts)]
pub struct SnapshotFrameDelete<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
}

pub fn handler(ctx: Context<SnapshotFrameDelete>) -> Result<()> {
    Ok(())
}
