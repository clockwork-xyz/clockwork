use {
    crate::state::*,
    anchor_lang::{
        prelude::*,
        solana_program::{instruction::Instruction, system_program, sysvar},
    },
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{Mint, Token, TokenAccount},
    },
    clockwork_scheduler::{
        program::ClockworkScheduler,
        state::{Queue, QueueStatus, SEED_TASK}
    },
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct NodeRegister<'info> {
    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(seeds = [SEED_AUTHORITY], bump)]
    pub authority: Box<Account<'info, Authority>>,

    #[account(
        has_one = authority, 
        constraint = cleanup_queue.name.eq("cleanup"),
        constraint = cleanup_queue.status == QueueStatus::Pending
    )]
    pub cleanup_queue: Box<Account<'info, Queue>>,

    #[account(
        seeds = [SEED_TASK, cleanup_queue.key().as_ref(), cleanup_queue.task_count.to_be_bytes().as_ref()],
        seeds::program = clockwork_scheduler::ID,
        bump
    )]
    pub cleanup_task: SystemAccount<'info>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Box<Account<'info, Config>>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT_ENTRY,
            snapshot.key().as_ref(),
            snapshot.node_count.to_be_bytes().as_ref(),
        ],
        bump,
        payer = owner,
        space = 8 + size_of::<SnapshotEntry>(),
    )]
    pub entry: Account<'info, SnapshotEntry>,

    #[account(address = config.mint)]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [
            SEED_NODE,
            worker.key().as_ref()
        ],
        bump,
        payer = owner,
        space = 8 + size_of::<Node>(),
    )]
    pub node: Account<'info, Node>,

    #[account(mut, constraint = owner.key() != worker.key())]
    pub owner: Signer<'info>,

    #[account(
        mut, 
        seeds = [SEED_REGISTRY], 
        bump,
        constraint = !registry.is_locked
    )]
    pub registry: Account<'info, Registry>,

    #[account(address = sysvar::rent::ID)]
    pub rent: Sysvar<'info, Rent>,

    #[account(address = clockwork_scheduler::ID)]
    pub scheduler_program: Program<'info, ClockworkScheduler>,

    #[account(
        mut,
        seeds = [
            SEED_SNAPSHOT,
            snapshot.id.to_be_bytes().as_ref(),
        ],
        bump,
        constraint = snapshot.status == SnapshotStatus::Current
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(
        has_one = authority, 
        constraint = snapshot_queue.name.eq("snapshot"),
        constraint = snapshot_queue.status == QueueStatus::Pending
    )]
    pub snapshot_queue: Box<Account<'info, Queue>>,

    #[account(
        seeds = [SEED_TASK, snapshot_queue.key().as_ref(), snapshot_queue.task_count.to_be_bytes().as_ref()],
        seeds::program = clockwork_scheduler::ID,
        bump
    )]
    pub snapshot_task: SystemAccount<'info>,

    #[account(
        init,
        payer = owner,
        associated_token::authority = node,
        associated_token::mint = mint,
    )]
    pub stake: Account<'info, TokenAccount>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(address = anchor_spl::token::ID)]
    pub token_program: Program<'info, Token>,

    #[account()]
    pub worker: Signer<'info>,
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, NodeRegister<'info>>) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let cleanup_queue = &ctx.accounts.cleanup_queue;
    let cleanup_task = &ctx.accounts.cleanup_task;
    let config = &ctx.accounts.config;
    let entry = &mut ctx.accounts.entry;
    let node = &mut ctx.accounts.node;
    let owner = &mut ctx.accounts.owner;
    let registry = &mut ctx.accounts.registry;
    let scheduler_program = &ctx.accounts.scheduler_program;
    let snapshot = &mut ctx.accounts.snapshot;
    let snapshot_queue = &ctx.accounts.snapshot_queue;
    let snapshot_task = &ctx.accounts.snapshot_task;
    let system_program = &ctx.accounts.system_program;
    let stake = &mut ctx.accounts.stake;
    let worker = &ctx.accounts.worker;

    // Get bumps
    let authority_bump = *ctx.bumps.get("authority").unwrap();

    // Add node to the registry
    registry.new_node(owner, node, stake, worker)?;

    // Add an empty entry to the current snapshot
    snapshot.capture(entry, node, stake)?;

    // Add an task to the snapshot queue to capture an entry for this node
    let current_snapshot_pubkey = Snapshot::pubkey(registry.snapshot_count.checked_sub(1).unwrap());
    let next_snapshot_pubkey = Snapshot::pubkey(registry.snapshot_count);
    let next_entry_pubkey = SnapshotEntry::pubkey(next_snapshot_pubkey, node.id);
    let snapshot_capture_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority.key(), false),
            AccountMeta::new_readonly(config.key(), false),
            AccountMeta::new(next_entry_pubkey, false),
            AccountMeta::new_readonly(node.key(), false,),
            AccountMeta::new(clockwork_scheduler::payer::ID, true),
            AccountMeta::new_readonly(snapshot_queue.key(), true),
            AccountMeta::new_readonly(registry.key(), false),
            AccountMeta::new(next_snapshot_pubkey, false),
            AccountMeta::new_readonly(stake.key(), false),
            AccountMeta::new_readonly(system_program.key(), false)
        ],
        data: clockwork_scheduler::anchor::sighash("snapshot_capture").into(),
    };
    let snapshot_rotate_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority.key(), false),
            AccountMeta::new_readonly(config.key(), false),
            AccountMeta::new(current_snapshot_pubkey, false),
            AccountMeta::new(next_snapshot_pubkey, false),
            AccountMeta::new_readonly(snapshot_queue.key(), true),
            AccountMeta::new(registry.key(), false),
        ],
        data: clockwork_scheduler::anchor::sighash("snapshot_rotate").into(),
    };
    clockwork_scheduler::cpi::task_new(
        CpiContext::new_with_signer(
            scheduler_program.to_account_info(),
            clockwork_scheduler::cpi::accounts::TaskNew {
                authority: authority.to_account_info(),
                payer: owner.to_account_info(),
                queue: snapshot_queue.to_account_info(),
                system_program: system_program.to_account_info(),
                task: snapshot_task.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[authority_bump]]],
        ),
        vec![snapshot_capture_ix.into(), snapshot_rotate_ix.into()],
    )?;

    // Add task to the cleanup queue to close the entry for this node
    let entry_close_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority.key(), false),
            AccountMeta::new(entry.key(), false),
            AccountMeta::new(cleanup_queue.key(), true),
            AccountMeta::new(snapshot.key(), false),
        ],
        data: clockwork_scheduler::anchor::sighash("entry_close").into(),
    };
    clockwork_scheduler::cpi::task_new(
        CpiContext::new_with_signer(
            scheduler_program.to_account_info(),
            clockwork_scheduler::cpi::accounts::TaskNew {
                authority: authority.to_account_info(),
                payer: owner.to_account_info(),
                queue: cleanup_queue.to_account_info(),
                system_program: system_program.to_account_info(),
                task: cleanup_task.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[authority_bump]]],
        ),
        vec![entry_close_ix.into()],
    )?;

    Ok(())
}
