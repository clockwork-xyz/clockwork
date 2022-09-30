use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::instruction::Instruction},
    clockwork_queue_program::state::{CrankResponse, Queue, SEED_QUEUE},
};

#[derive(Accounts)]
pub struct SnapshotRotate<'info> {
    #[account(seeds = [SEED_AUTHORITY], bump)]
    pub authority: Account<'info, Authority>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            current_snapshot.id.to_be_bytes().as_ref()
        ],
        bump,
        constraint = current_snapshot.status == SnapshotStatus::Current
    )]
    pub current_snapshot: Account<'info, Snapshot>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            current_snapshot.id.checked_add(1).unwrap().to_be_bytes().as_ref()
        ],
        bump,
    )]
    pub next_snapshot: Account<'info, Snapshot>,

    #[account(mut, seeds = [SEED_REGISTRY], bump)]
    pub registry: Account<'info, Registry>,

    #[account(
        signer, 
        seeds = [
            SEED_QUEUE, 
            authority.key().as_ref(), 
            "snapshot".as_bytes()
        ], 
        seeds::program = clockwork_queue_program::ID,
        bump,
        has_one = authority
    )]
    pub snapshot_queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<SnapshotRotate>) -> Result<CrankResponse> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let current_snapshot = &mut ctx.accounts.current_snapshot;
    let next_snapshot = &mut ctx.accounts.next_snapshot;
    let registry = &mut ctx.accounts.registry;
    let snapshot_queue = &ctx.accounts.snapshot_queue;

    // Rotate the snapshot
    registry.rotate_snapshot(Some(current_snapshot), next_snapshot)?;

    // Build the next instruction
    let next_instruction = Some(
        Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new_readonly(authority.key(), false),
                AccountMeta::new(current_snapshot.key(), false),
                AccountMeta::new(snapshot_queue.key(), true),
            ],
            data: clockwork_queue_program::anchor::sighash("snapshot_close").into(),
        }.into()
    );

    Ok(CrankResponse { next_instruction })
}
