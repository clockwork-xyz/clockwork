use {
    crate::state::*,
    anchor_lang::{
        prelude::*, 
        solana_program::{instruction::Instruction, native_token::LAMPORTS_PER_SOL, system_program}
    },
    anchor_spl::token::Mint,
    cronos_scheduler::state::{SEED_QUEUE, SEED_TASK},
    std::mem::size_of,
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        seeds = [SEED_AUTHORITY],
        bump,
        payer = admin,
        space = 8 + size_of::<Authority>(),
    )]
    pub authority: Account<'info, Authority>,

    #[account(
        seeds = [
            SEED_QUEUE, 
            authority.key().as_ref(), 
            "cleanup".as_bytes()
        ], 
        seeds::program = cronos_scheduler::ID,
        bump, 
    )]
    pub cleanup_queue: SystemAccount<'info>,

    #[account(
        seeds = [
            SEED_TASK, 
            cleanup_queue.key().as_ref(), 
            (0 as u64).to_be_bytes().as_ref()
        ], 
        seeds::program = cronos_scheduler::ID,
        bump, 
    )]
    pub cleanup_task: SystemAccount<'info>,

    #[account(
        init,
        seeds = [SEED_CONFIG],
        bump,
        payer = admin,
        space = 8 + size_of::<Config>(),
    )]
    pub config: Account<'info, Config>,
    
    #[account(
        init,
        seeds = [SEED_ROTATOR],
        bump,
        payer = admin,
        space = 8 + size_of::<Rotator>(),
    )]
    pub rotator: Account<'info, Rotator>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        seeds = [SEED_REGISTRY],
        bump,
        payer = admin,
        space = 8 + size_of::<Registry>(),
    )]
    pub registry: Account<'info, Registry>,
 
    #[account(address = cronos_scheduler::ID)]
    pub scheduler_program: Program<'info, cronos_scheduler::program::CronosScheduler>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT, 
            (0 as u64).to_be_bytes().as_ref()
        ],
        bump,
        payer = admin,
        space = 8 + size_of::<Snapshot>(),
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account(
        seeds = [
            SEED_QUEUE, 
            authority.key().as_ref(), 
            "snapshot".as_bytes()
        ], 
        seeds::program = cronos_scheduler::ID,
        bump
    )]
    pub snapshot_queue: SystemAccount<'info>,

    #[account(
        seeds = [
            SEED_TASK, 
            snapshot_queue.key().as_ref(), 
            (0 as u64).to_be_bytes().as_ref()
        ], 
        seeds::program = cronos_scheduler::ID,
        bump
    )]
    pub snapshot_task: SystemAccount<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,    
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
    // Get accounts
    let admin = &ctx.accounts.admin;
    let authority = &ctx.accounts.authority;
    let cleanup_queue = &ctx.accounts.cleanup_queue;
    let cleanup_task = &ctx.accounts.cleanup_task;
    let config = &mut ctx.accounts.config;
    let rotator = &mut ctx.accounts.rotator;
    let mint = &ctx.accounts.mint;
    let registry = &mut ctx.accounts.registry;
    let scheduler_program = &ctx.accounts.scheduler_program;
    let snapshot = &mut ctx.accounts.snapshot;
    let snapshot_queue = &ctx.accounts.snapshot_queue;
    let snapshot_task = &ctx.accounts.snapshot_task;
    let system_program = &ctx.accounts.system_program;

    // Initialize accounts
    config.new(admin.key(), mint.key())?;
    registry.new()?;
    rotator.new()?;

    // Setup the first snapshot
    registry.new_snapshot(snapshot)?;
    registry.rotate_snapshot(None, snapshot)?;

    // Create a queue to take snapshots of the registry
    let bump = *ctx.bumps.get("authority").unwrap();
    cronos_scheduler::cpi::queue_new(
        CpiContext::new_with_signer(
            scheduler_program.to_account_info(),
            cronos_scheduler::cpi::accounts::QueueNew {
                authority: authority.to_account_info(),
                payer: admin.to_account_info(),
                queue: snapshot_queue.to_account_info(),
                system_program: system_program.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[bump]]]
        ),
        LAMPORTS_PER_SOL,
        "snapshot".into(),
        "0 * * * * * *".into()
    )?;

    // Add an task to the snapshot queue to kick things off
    let next_snapshot_pubkey = Snapshot::pubkey(1);
    let snapshot_start_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority.key(), false),
            AccountMeta::new_readonly(config.key(), false),
            AccountMeta::new(cronos_scheduler::payer::ID, true),
            AccountMeta::new_readonly(snapshot_queue.key(), true),
            AccountMeta::new(registry.key(), false),
            AccountMeta::new(next_snapshot_pubkey, false),
            AccountMeta::new_readonly(system_program.key(), false),
        ],
        data: cronos_scheduler::anchor::sighash("snapshot_start").into(),
    };
    let snapshot_rotate_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority.key(), false),
            AccountMeta::new_readonly(config.key(), false),
            AccountMeta::new(snapshot.key(), false),
            AccountMeta::new(next_snapshot_pubkey, false),
            AccountMeta::new_readonly(snapshot_queue.key(), true),
            AccountMeta::new(registry.key(), false),
        ],
        data: cronos_scheduler::anchor::sighash("snapshot_rotate").into(),
    };
    cronos_scheduler::cpi::task_new(
        CpiContext::new_with_signer(
            scheduler_program.to_account_info(),
            cronos_scheduler::cpi::accounts::TaskNew {
                authority: authority.to_account_info(),
                payer: admin.to_account_info(),
                queue: snapshot_queue.to_account_info(),
                system_program: system_program.to_account_info(),
                task: snapshot_task.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[bump]]],
        ),
        vec![snapshot_start_ix.into(), snapshot_rotate_ix.into()],
    )?;

    // Create a queue to cleanup old snapshots
    cronos_scheduler::cpi::queue_new(
        CpiContext::new_with_signer(
            scheduler_program.to_account_info(),
            cronos_scheduler::cpi::accounts::QueueNew {
                authority: authority.to_account_info(),
                payer: admin.to_account_info(),
                queue: cleanup_queue.to_account_info(),
                system_program: system_program.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[bump]]]
        ),
        LAMPORTS_PER_SOL,
        "cleanup".into(),
        "0 * * * * * *".into()
    )?;

    // Create task to close archived snapshot
    let snapshot_close_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority.key(), false),
            AccountMeta::new(cleanup_queue.key(), true),
            AccountMeta::new(snapshot.key(), false),
        ],
        data: cronos_scheduler::anchor::sighash("snapshot_close").into(),
    };
    cronos_scheduler::cpi::task_new(
        CpiContext::new_with_signer(
            scheduler_program.to_account_info(),
            cronos_scheduler::cpi::accounts::TaskNew {
                authority: authority.to_account_info(),
                payer: admin.to_account_info(),
                queue: cleanup_queue.to_account_info(),
                system_program: system_program.to_account_info(),
                task: cleanup_task.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[bump]]],
        ),
        vec![snapshot_close_ix.into()],
    )?;


    Ok(())
}

