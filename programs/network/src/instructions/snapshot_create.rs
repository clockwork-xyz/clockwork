use {
    crate::objects::*,
    anchor_lang::{prelude::*, solana_program::instruction::Instruction},
    anchor_spl::associated_token::get_associated_token_address,
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct SnapshotCreate<'info> {
    #[account(address = Authority::pubkey())]
    pub authority: Box<Account<'info, Authority>>,

    #[account(address = Config::pubkey())]
    pub config: Box<Account<'info, Config>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, seeds = [SEED_REGISTRY], bump)]
    pub registry: Account<'info, Registry>,

    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT,
            registry.snapshot_count.to_be_bytes().as_ref(),
        ],
        bump,
        space = 8 + size_of::<Snapshot>(),
        payer = payer
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SnapshotCreate>) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let config = &ctx.accounts.config;
    let registry = &mut ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;
    let signer = &ctx.accounts.signer;
    let system_program = &ctx.accounts.system_program;

    // Start a new snapshot
    registry.new_snapshot(snapshot)?;

    Ok(())

    // Build the next crank instruction
    // let next_instruction = if registry.node_count > 0 {
    //     // There are nodes in the registry. Begin creating snapshot entries.
    //     let node_pubkey = Node::pubkey(0);
    //     let entry_pubkey = SnapshotEntry::pubkey(snapshot.key(), 0);
    //     let stake_pubkey = get_associated_token_address(&node_pubkey, &config.mint);
    //     Some(
    //         Instruction {
    //             program_id: crate::ID,
    //             accounts: vec![
    //                 AccountMeta::new_readonly(authority.key(), false),
    //                 AccountMeta::new_readonly(config.key(), false),
    //                 AccountMeta::new(entry_pubkey, false),
    //                 AccountMeta::new_readonly(node_pubkey, false),
    //                 AccountMeta::new(clockwork_queue_program::utils::PAYER_PUBKEY, true),
    //                 AccountMeta::new_readonly(registry.key(), false),
    //                 AccountMeta::new(snapshot.key(), false),
    //                 AccountMeta::new_readonly(snapshot_queue.key(), true),
    //                 AccountMeta::new_readonly(stake_pubkey, false),
    //                 AccountMeta::new_readonly(system_program.key(), false),
    //             ],
    //             data: clockwork_queue_program::utils::anchor_sighash("entry_create").into(),
    //         }
    //         .into(),
    //     )
    // } else {
    //     // There are no nodes in the registry. Activate the new snapshot.
    //     Some(
    //         Instruction {
    //             program_id: crate::ID,
    //             accounts: vec![
    //                 AccountMeta::new_readonly(authority.key(), false),
    //                 AccountMeta::new_readonly(config.key(), false),
    //                 AccountMeta::new(Snapshot::pubkey(snapshot.id.checked_sub(1).unwrap()), false), // The current active snapshot
    //                 AccountMeta::new(snapshot.key(), false), // The next active snapshot
    //                 AccountMeta::new(registry.key(), false),
    //                 AccountMeta::new_readonly(snapshot_queue.key(), true),
    //             ],
    //             data: clockwork_queue_program::utils::anchor_sighash("snapshot_rotate").into(),
    //         }
    //         .into(),
    //     )
    // };

    // Ok(CrankResponse { next_instruction })
}
