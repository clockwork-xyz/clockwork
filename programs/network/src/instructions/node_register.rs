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
    cronos_scheduler::{
        program::CronosScheduler,
        state::{Queue, Manager}
    },
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct NodeRegister<'info> {
    #[account(address = anchor_spl::associated_token::ID)]
    pub associated_token_program: Program<'info, AssociatedToken>,

    #[account(seeds = [SEED_AUTHORITY], bump, has_one = manager)]
    pub authority: Box<Account<'info, Authority>>,

    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Box<Account<'info, Config>>,

    // #[account(has_one = manager)]
    // pub cycler_queue: Box<Account<'info, Queue>>,

    #[account()]
    pub delegate: Signer<'info>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT_ENTRY,
            snapshot.key().as_ref(),
            snapshot.entry_count.to_be_bytes().as_ref(),
        ],
        bump,
        payer = owner,
        space = 8 + size_of::<SnapshotEntry>(),
    )]
    pub entry: Account<'info, SnapshotEntry>,

    #[account(constraint = manager.authority == authority.key())]
    pub manager: Box<Account<'info, Manager>>,

    #[account(address = config.mint)]
    pub mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [
            SEED_NODE,
            delegate.key().as_ref()
        ],
        bump,
        payer = owner,
        space = 8 + size_of::<Node>(),
    )]
    pub node: Account<'info, Node>,

    #[account(mut, constraint = owner.key() != delegate.key())]
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

    #[account(address = cronos_scheduler::ID)]
    pub scheduler_program: Program<'info, CronosScheduler>,

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

    #[account(has_one = manager)]
    pub snapshot_queue: Box<Account<'info, Queue>>,

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
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, NodeRegister<'info>>) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let config = &ctx.accounts.config;
    // let cycler_queue = &ctx.accounts.cycler_queue;
    let delegate = &ctx.accounts.delegate;
    let entry = &mut ctx.accounts.entry;
    let manager = &ctx.accounts.manager;
    let node = &mut ctx.accounts.node;
    let owner = &mut ctx.accounts.owner;
    let registry = &mut ctx.accounts.registry;
    let scheduler_program = &ctx.accounts.scheduler_program;
    let snapshot = &mut ctx.accounts.snapshot;
    let snapshot_queue = &ctx.accounts.snapshot_queue;
    let system_program = &ctx.accounts.system_program;
    let stake = &mut ctx.accounts.stake;

    // Get remaining accountsgs
    // let cycler_task = ctx.remaining_accounts.get(0).unwrap();
    let snapshot_task = ctx.remaining_accounts.get(0).unwrap();
    
    // Get bumps
    let authority_bump = *ctx.bumps.get("authority").unwrap();

    // Add node to the registry
    registry.new_node(delegate, owner, node, stake)?;

    // Add an empty entry to the current snapshot
    snapshot.capture(entry, node, stake)?;

    // Add an task to the cycler queue to check the snapshot entry for this node
    // let cycler_run_ix = Instruction {
    //     program_id: crate::ID,
    //     accounts: vec![
    //         AccountMeta::new_readonly(authority.key(), false),
    //         AccountMeta::new(Cycler::pda().0, false),
    //         AccountMeta::new_readonly(entry.key(), false),
    //         AccountMeta::new(cronos_pool::state::Pool::pda().0, false),
    //         AccountMeta::new_readonly(cronos_pool::state::Config::pda().0, false),
    //         AccountMeta::new_readonly(cronos_pool::ID, false),
    //         AccountMeta::new_readonly(manager.key(), true),
    //         AccountMeta::new_readonly(registry.key(), false),
    //         AccountMeta::new_readonly(snapshot.key(), false),
    //     ],
    //     data: cronos_scheduler::anchor::sighash("cycler_run").into(),
    // };
    // cronos_scheduler::cpi::task_new(
    //     CpiContext::new_with_signer(
    //         scheduler_program.to_account_info(),
    //         cronos_scheduler::cpi::accounts::TaskNew {
    //             authority: authority.to_account_info(),
    //             manager: manager.to_account_info(),
    //             payer: owner.to_account_info(),
    //             queue: cycler_queue.to_account_info(),
    //             system_program: system_program.to_account_info(),
    //             task: cycler_task.to_account_info(),
    //         },
    //         &[&[SEED_AUTHORITY, &[authority_bump]]],
    //     ),
    //     vec![cycler_run_ix.into()],
    // )?;

    // Add an task to the snapshot queue to capture an entry for this node
    let current_snapshot_pubkey = Snapshot::pda(registry.snapshot_count.checked_sub(1).unwrap()).0;
    let next_snapshot_pubkey = Snapshot::pda(registry.snapshot_count).0;
    let next_entry_pubkey = SnapshotEntry::pda(next_snapshot_pubkey, node.id).0;
    let stake_pubkey = stake.key();
    let snapshot_capture_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority.key(), false),
            AccountMeta::new_readonly(config.key(), false),
            AccountMeta::new(next_entry_pubkey, false),
            AccountMeta::new_readonly(node.key(), false,),
            AccountMeta::new(cronos_scheduler::payer::ID, true),
            AccountMeta::new_readonly(manager.key(), true),
            AccountMeta::new_readonly(registry.key(), false),
            AccountMeta::new(next_snapshot_pubkey, false),
            AccountMeta::new_readonly(stake_pubkey, false),
            AccountMeta::new_readonly(system_program.key(), false)
        ],
        data: cronos_scheduler::anchor::sighash("snapshot_capture").into(),
    };
    let snapshot_rotate_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority.key(), false),
            AccountMeta::new_readonly(sysvar::clock::ID, false),
            AccountMeta::new_readonly(config.key(), false),
            AccountMeta::new(current_snapshot_pubkey, false),
            AccountMeta::new(next_snapshot_pubkey, false),
            AccountMeta::new_readonly(manager.key(), true),
            AccountMeta::new(registry.key(), false),
        ],
        data: cronos_scheduler::anchor::sighash("snapshot_rotate").into(),
    };
    cronos_scheduler::cpi::task_new(
        CpiContext::new_with_signer(
            scheduler_program.to_account_info(),
            cronos_scheduler::cpi::accounts::TaskNew {
                authority: authority.to_account_info(),
                manager: manager.to_account_info(),
                payer: owner.to_account_info(),
                queue: snapshot_queue.to_account_info(),
                system_program: system_program.to_account_info(),
                task: snapshot_task.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[authority_bump]]],
        ),
        vec![snapshot_capture_ix.into(), snapshot_rotate_ix.into()],
    )?;

    Ok(())
}
