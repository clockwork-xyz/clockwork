use {
    crate::objects::*,
    anchor_lang::{prelude::*, solana_program::instruction::Instruction},
    clockwork_queue_program::objects::{CrankResponse, Queue, QueueAccount},
};

#[derive(Accounts)]
pub struct SnapshotRotate<'info> {
    #[account(address = Authority::pubkey())]
    pub authority: Account<'info, Authority>,

    #[account(address = Config::pubkey())]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        address = current_snapshot.pubkey(),
        constraint = current_snapshot.status == SnapshotStatus::Current
    )]
    pub current_snapshot: Account<'info, Snapshot>,

    #[account(
        mut,
        address = next_snapshot.pubkey(),
        constraint = current_snapshot.id.checked_add(1).unwrap().eq(&next_snapshot.id)
    )]
    pub next_snapshot: Account<'info, Snapshot>,

    #[account(mut, address = Registry::pubkey())]
    pub registry: Account<'info, Registry>,

    #[account(
        address = snapshot_queue.pubkey(),
        constraint = snapshot_queue.id.eq("snapshot"),
        has_one = authority,
        signer,
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
            data: clockwork_queue_program::utils::anchor_sighash("snapshot_close").into(),
        }
        .into(),
    );

    Ok(CrankResponse { next_instruction })
}
