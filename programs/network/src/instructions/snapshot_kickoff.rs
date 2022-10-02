use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::instruction::Instruction, system_program},
    clockwork_queue_program::objects::{CrankResponse, Queue, QueueAccount},
};

#[derive(Accounts)]
pub struct SnapshotKickoff<'info> {
    #[account(seeds = [SEED_AUTHORITY], bump)]
    pub authority: Box<Account<'info, Authority>>,

    #[account(mut, seeds = [SEED_REGISTRY], bump)]
    pub registry: Account<'info, Registry>,

    #[account(
        address = snapshot_queue.pubkey(),
        constraint = snapshot_queue.id.eq("snapshot"),
        has_one = authority,
        signer,
    )]
    pub snapshot_queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<SnapshotKickoff>) -> Result<CrankResponse> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let registry = &mut ctx.accounts.registry;
    let snapshot_queue = &ctx.accounts.snapshot_queue;

    // Build the next crank instruction
    Ok(CrankResponse {
        next_instruction: Some(
            Instruction {
                program_id: crate::ID,
                accounts: vec![
                    AccountMeta::new_readonly(authority.key(), false),
                    AccountMeta::new_readonly(Config::pubkey(), false),
                    AccountMeta::new(clockwork_queue_program::utils::PAYER_PUBKEY, true),
                    AccountMeta::new(registry.key(), false),
                    AccountMeta::new(Snapshot::pubkey(registry.snapshot_count), false),
                    AccountMeta::new_readonly(snapshot_queue.key(), true),
                    AccountMeta::new_readonly(system_program::ID, false),
                ],
                data: clockwork_queue_program::utils::anchor_sighash("snapshot_create").into(),
            }
            .into(),
        ),
    })
}
