use {
    crate::state::*,
    anchor_lang::prelude::*,
    clockwork_queue_program::objects::{Queue, QueueAccount},
};

#[derive(Accounts)]
pub struct SnapshotPause<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(seeds = [SEED_AUTHORITY], bump)]
    pub authority: Account<'info, Authority>,

    #[account(address = clockwork_queue_program::ID)]
    pub clockwork_program: Program<'info, clockwork_queue_program::program::QueueProgram>,

    #[account(seeds = [SEED_CONFIG], bump, has_one = admin)]
    pub config: Account<'info, Config>,

    #[account(
        address = snapshot_queue.pubkey(),
        constraint = snapshot_queue.id.eq("snapshot"),
        has_one = authority,
        signer,
    )]
    pub snapshot_queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<SnapshotPause>) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let clockwork_program = &ctx.accounts.clockwork_program;
    let snapshot_queue = &ctx.accounts.snapshot_queue;

    // Pause the snapshot queue
    let bump = *ctx.bumps.get("authority").unwrap();
    clockwork_queue_program::cpi::queue_pause(CpiContext::new_with_signer(
        clockwork_program.to_account_info(),
        clockwork_queue_program::cpi::accounts::QueuePause {
            authority: authority.to_account_info(),
            queue: snapshot_queue.to_account_info(),
        },
        &[&[SEED_AUTHORITY, &[bump]]],
    ))?;

    Ok(())
}
