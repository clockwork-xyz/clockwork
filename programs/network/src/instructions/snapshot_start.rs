use anchor_spl::associated_token::get_associated_token_address;

use {
    crate::state::*,
    anchor_lang::{prelude::*, solana_program::instruction::Instruction},
    clockwork_crank::state::{CrankResponse, Queue, SEED_QUEUE},
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct SnapshotStart<'info> {
    #[account(seeds = [SEED_AUTHORITY], bump)]
    pub authority: Box<Account<'info, Authority>>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, seeds = [SEED_REGISTRY], bump)]
    pub registry: Account<'info, Registry>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT,
            registry.snapshot_count.to_be_bytes().as_ref()
        ],
        bump,
        space = 8 + size_of::<Snapshot>(),
        payer = payer
    )]
    pub snapshot: Account<'info, Snapshot>,

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

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SnapshotStart>) -> Result<CrankResponse> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let config = &ctx.accounts.config;
    let registry = &mut ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;
    let snapshot_queue = &ctx.accounts.snapshot_queue;
    let system_program = &ctx.accounts.system_program;

    // Start a new snapshot
    registry.new_snapshot(snapshot)?;

    // Build the next crank instruction
    let next_instruction = if registry.node_count > 0 {
        // There are nodes in the registry. Begin creating snapshot entries.
        let node_pubkey = Node::pubkey(0);
        let entry_pubkey = SnapshotEntry::pubkey(snapshot.key(), 0);
        let stake_pubkey = get_associated_token_address(&entry_pubkey, &config.mint);
        Some(
             Instruction {
                program_id: crate::ID,
                accounts: vec![
                    AccountMeta::new_readonly(authority.key(), false),
                    AccountMeta::new_readonly(config.key(), false),
                    AccountMeta::new(entry_pubkey, false),
                    AccountMeta::new_readonly(node_pubkey, false),
                    AccountMeta::new(clockwork_crank::payer::ID, true),
                    AccountMeta::new_readonly(snapshot_queue.key(), true),
                    AccountMeta::new_readonly(registry.key(), false),
                    AccountMeta::new(snapshot.key(), false),
                    AccountMeta::new_readonly(stake_pubkey, false),
                    AccountMeta::new_readonly(system_program.key(), false)
                ],
                data: clockwork_crank::anchor::sighash("entry_create").into(),
            }
            .into()
        )
    } else {
        // There are no nodes in the registry. Activate the new snapshot.
        Some(
            Instruction {
                program_id: crate::ID,
                accounts: vec![
                    AccountMeta::new_readonly(authority.key(), false),
                    AccountMeta::new_readonly(config.key(), false),
                    AccountMeta::new(Snapshot::pubkey(snapshot.id.checked_sub(1).unwrap()), false), // The current active snapshot
                    AccountMeta::new(snapshot.key(), false), // The next active snapshot
                    AccountMeta::new(registry.key(), false),
                    AccountMeta::new_readonly(snapshot_queue.key(), true),
                ],
                data: clockwork_crank::anchor::sighash("snapshot_rotate").into(),
            }
            .into()
        )
    };

    Ok(CrankResponse { next_instruction })
}
