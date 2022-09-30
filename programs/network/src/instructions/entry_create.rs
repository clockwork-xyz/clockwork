use {
    crate::{errors::ClockworkError, state::*},
    anchor_lang::{prelude::*, solana_program::{instruction::Instruction, system_program}},
    anchor_spl::{associated_token::get_associated_token_address, token::TokenAccount},
    clockwork_queue_program::state::{CrankResponse, Queue, SEED_QUEUE},
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct EntryCreate<'info> {
    #[account(seeds = [SEED_AUTHORITY], bump)]
    pub authority: Box<Account<'info, Authority>>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT_ENTRY,
            snapshot.key().as_ref(),
            snapshot.node_count.to_be_bytes().as_ref()
        ],
        bump,
        payer = payer,
        space = 8 + size_of::<SnapshotEntry>(),
    )]
    pub entry: Account<'info, SnapshotEntry>,

    #[account(
        seeds = [
            SEED_NODE,
            node.id.to_be_bytes().as_ref(),
        ],
        bump,
        constraint = node.id == snapshot.node_count @ ClockworkError::InvalidNode
    )]
    pub node: Box<Account<'info, Node>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(seeds = [SEED_REGISTRY], bump)]
    pub registry: Box<Account<'info, Registry>>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            snapshot.id.to_be_bytes().as_ref()
        ],
        bump,
        constraint = snapshot.status == SnapshotStatus::InProgress @ ClockworkError::SnapshotNotInProgress,
        constraint = snapshot.node_count < registry.node_count,
    )]
    pub snapshot: Account<'info, Snapshot>,

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

    #[account(
        associated_token::authority = node,
        associated_token::mint = config.mint,
    )]
    pub stake: Account<'info, TokenAccount>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<EntryCreate>) -> Result<CrankResponse> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let config = &ctx.accounts.config;
    let entry = &mut ctx.accounts.entry;
    let node = &ctx.accounts.node;
    let registry = &ctx.accounts.registry;
    let stake = &ctx.accounts.stake;
    let snapshot = &mut ctx.accounts.snapshot;
    let snapshot_queue = &ctx.accounts.snapshot_queue;
    let system_program = &ctx.accounts.system_program;

    // Capture the snapshot entry
    snapshot.capture(entry, node, stake)?;

    // Build the next crank instruction
    let next_instruction = if snapshot.node_count < registry.node_count {
        // There are more nodes in the registry. Continue creating snapshot entries.
        let next_id = node.id.checked_add(1).unwrap();
        let next_node_pubkey = Node::pubkey(next_id); 
        let next_entry_pubkey = SnapshotEntry::pubkey(snapshot.key(), next_id);
        let stake_pubkey = get_associated_token_address(&next_node_pubkey, &config.mint);
        Some(
             Instruction {
                program_id: crate::ID,
                accounts: vec![
                    AccountMeta::new_readonly(authority.key(), false),
                    AccountMeta::new_readonly(config.key(), false),
                    AccountMeta::new(next_entry_pubkey, false),
                    AccountMeta::new_readonly(next_node_pubkey, false),
                    AccountMeta::new(clockwork_queue_program::utils::PAYER_PUBKEY, true),
                    AccountMeta::new_readonly(registry.key(), false),
                    AccountMeta::new(snapshot.key(), false),
                    AccountMeta::new_readonly(snapshot_queue.key(), true),
                    AccountMeta::new_readonly(stake_pubkey, false),
                    AccountMeta::new_readonly(system_program.key(), false)
                ],
                data: clockwork_queue_program::utils::anchor_sighash("entry_create").into(),
            }
            .into()
        )
    } else {
        // We have created entries for all the nodes. Activate the new snapshot.
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
                data: clockwork_queue_program::utils::anchor_sighash("snapshot_rotate").into(),
            }
            .into()
        )
    };

    Ok(CrankResponse { next_instruction })
}
