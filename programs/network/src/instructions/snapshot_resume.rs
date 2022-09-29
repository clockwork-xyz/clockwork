use {
    crate::state::*,
    anchor_lang::{prelude::*},
    clockwork_queue::state::{Queue, SEED_QUEUE},
};

#[derive(Accounts)]
pub struct SnapshotResume<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(seeds = [SEED_AUTHORITY], bump)]
    pub authority: Account<'info, Authority>,

    #[account(address = clockwork_queue::ID)]
    pub clockwork_program: Program<'info, clockwork_queue::program::ClockworkQueue>,

    #[account(seeds = [SEED_CONFIG], bump, has_one = admin)]
    pub config: Account<'info, Config>,

    #[account(
        seeds = [
            SEED_QUEUE, 
            authority.key().as_ref(), 
            "snapshot".as_bytes()
        ], 
        seeds::program = clockwork_queue::ID,
        bump,
    )]
    pub snapshot_queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<SnapshotResume>) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let clockwork_program = &ctx.accounts.clockwork_program;
    let snapshot_queue = &ctx.accounts.snapshot_queue;

    // Pause the snapshot queue
    let bump = *ctx.bumps.get("authority").unwrap();
    clockwork_queue::cpi::queue_resume(
        CpiContext::new_with_signer(
            clockwork_program.to_account_info(),
            clockwork_queue::cpi::accounts::QueueResume {
                authority: authority.to_account_info(),
                queue: snapshot_queue.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[bump]]]
        ),
    )?;

    Ok(())
}
