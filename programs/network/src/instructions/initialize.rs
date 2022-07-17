use {
    crate::state::*,
    anchor_lang::{
        prelude::*, 
        solana_program::{instruction::Instruction, native_token::LAMPORTS_PER_SOL, system_program}
    },
    anchor_spl::token::Mint,
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

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
    // Get accounts
    let admin = &ctx.accounts.admin;
    let authority = &mut ctx.accounts.authority;
    let rotator = &mut ctx.accounts.rotator;
    let config = &mut ctx.accounts.config;
    let mint = &ctx.accounts.mint;
    let registry = &mut ctx.accounts.registry;
    let scheduler_program = &ctx.accounts.scheduler_program;
    let snapshot = &mut ctx.accounts.snapshot;
    let system_program = &ctx.accounts.system_program;

    // Get remaining accounts
    let snapshot_queue = ctx.remaining_accounts.get(0).unwrap();
    let snapshot_task = ctx.remaining_accounts.get(1).unwrap();

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
        0,
        "0 * * * * * *".into()
    )?;

    // TOOD Create a queue to cleanup snapshots and snapshot entries
    // TODO Return the lamports to the authority

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

    Ok(())
}

