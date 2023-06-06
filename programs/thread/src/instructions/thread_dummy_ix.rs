use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ThreadDummyIx<'info> {
    /// CHECK: thread
    pub thread: UncheckedAccount<'info>,
}

pub fn handler (ctx: Context<ThreadDummyIx>) -> Result<()> {
  msg!("Hello, Clockwork Thread: {:#?}", ctx.accounts.thread.key);
  Ok(())
}

