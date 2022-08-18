use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::instruction::Instruction, system_program},
    clockwork_crank::state::{CrankResponse, Queue, SEED_QUEUE},
};

#[derive(Accounts)]
pub struct SnapshotKickoff<'info> {
    #[account(seeds = [SEED_AUTHORITY], bump)]
    pub authority: Box<Account<'info, Authority>>,

    #[account(mut, seeds = [SEED_REGISTRY], bump)]
    pub registry: Account<'info, Registry>,

    #[account(
        signer, 
        seeds = [
            SEED_QUEUE, 
            authority.key().as_ref(), 
            "snapshot".as_bytes()
        ], 
        seeds::program = clockwork_crank::ID,
        bump,
        has_one = authority
    )]
    pub snapshot_queue: Account<'info, Queue>,
}

pub fn handler(ctx: Context<SnapshotKickoff>) -> Result<CrankResponse> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let registry = &mut ctx.accounts.registry;
    let snapshot_queue = &ctx.accounts.snapshot_queue;

    // Build the next crank instruction
    Ok(CrankResponse { next_instruction:
        Some(Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new_readonly(authority.key(), false),
                AccountMeta::new_readonly(Config::pubkey(), false),
                AccountMeta::new(clockwork_crank::payer::ID, true),
                AccountMeta::new(registry.key(), false),
                AccountMeta::new(Snapshot::pubkey(registry.snapshot_count), false),
                AccountMeta::new_readonly(snapshot_queue.key(), true),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data: clockwork_crank::anchor::sighash("snapshot_create").into(),
        }.into()) 
    })
}
