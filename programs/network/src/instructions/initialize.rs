use clockwork_crank::state::Trigger;

use {
    crate::state::*,
    anchor_lang::{
        prelude::*, 
        solana_program::{instruction::Instruction, native_token::LAMPORTS_PER_SOL, system_program}
    },
    anchor_spl::token::Mint,
    clockwork_crank::state::{SEED_QUEUE},
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

    #[account(address = clockwork_crank::ID)]
    pub clockwork_program: Program<'info, clockwork_crank::program::ClockworkCrank>,

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
        seeds::program = clockwork_crank::ID,
        bump
    )]
    pub snapshot_queue: SystemAccount<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,    
}

pub fn handler<'info>(ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
    // Get accounts
    let admin = &ctx.accounts.admin;
    let authority = &ctx.accounts.authority;
    let clockwork_program = &ctx.accounts.clockwork_program;
    let config = &mut ctx.accounts.config;
    let rotator = &mut ctx.accounts.rotator;
    let mint = &ctx.accounts.mint;
    let registry = &mut ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;
    let snapshot_queue = &ctx.accounts.snapshot_queue;
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
    let next_snapshot_pubkey = Snapshot::pubkey(1);
    let snapshot_start_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority.key(), false),
            AccountMeta::new_readonly(config.key(), false),
            AccountMeta::new(clockwork_crank::payer::ID, true),
            AccountMeta::new_readonly(snapshot_queue.key(), true),
            AccountMeta::new(registry.key(), false),
            AccountMeta::new(next_snapshot_pubkey, false),
            AccountMeta::new_readonly(system_program.key(), false),
        ],
        data: clockwork_crank::anchor::sighash("snapshot_start").into(),
    };
    clockwork_crank::cpi::queue_create(
        CpiContext::new_with_signer(
            clockwork_program.to_account_info(),
            clockwork_crank::cpi::accounts::QueueCreate {
                authority: authority.to_account_info(),
                payer: admin.to_account_info(),
                queue: snapshot_queue.to_account_info(),
                system_program: system_program.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[bump]]]
        ),
        LAMPORTS_PER_SOL,
        snapshot_start_ix.into(),
        "snapshot".into(),
        Trigger::Cron { schedule: "0 * * * * * *".into() }
    )?;

    // Add an task to the snapshot queue to kick things off
    // let snapshot_start_ix = Instruction {
    //     program_id: crate::ID,
    //     accounts: vec![
    //         AccountMeta::new_readonly(authority.key(), false),
    //         AccountMeta::new_readonly(config.key(), false),
    //         AccountMeta::new(clockwork_crank::payer::ID, true),
    //         AccountMeta::new_readonly(snapshot_queue.key(), true),
    //         AccountMeta::new(registry.key(), false),
    //         AccountMeta::new(next_snapshot_pubkey, false),
    //         AccountMeta::new_readonly(system_program.key(), false),
    //     ],
    //     data: clockwork_crank::anchor::sighash("snapshot_start").into(),
    // };
    // let snapshot_rotate_ix = Instruction {
    //     program_id: crate::ID,
    //     accounts: vec![
    //         AccountMeta::new_readonly(authority.key(), false),
    //         AccountMeta::new_readonly(config.key(), false),
    //         AccountMeta::new(snapshot.key(), false),
    //         AccountMeta::new(next_snapshot_pubkey, false),
    //         AccountMeta::new_readonly(snapshot_queue.key(), true),
    //         AccountMeta::new(registry.key(), false),
    //     ],
    //     data: clockwork_crank::anchor::sighash("snapshot_rotate").into(),
    // };
    // clockwork_crank::cpi::task_new(
    //     CpiContext::new_with_signer(
    //         clockwork_program.to_account_info(),
    //         clockwork_crank::cpi::accounts::TaskNew {
    //             authority: authority.to_account_info(),
    //             payer: admin.to_account_info(),
    //             queue: snapshot_queue.to_account_info(),
    //             system_program: system_program.to_account_info(),
    //             task: snapshot_task.to_account_info(),
    //         },
    //         &[&[SEED_AUTHORITY, &[bump]]],
    //     ),
    //     vec![snapshot_start_ix.into(), snapshot_rotate_ix.into()],
    // )?;

    // Create a queue to cleanup old snapshots
    // let snapshot_close_ix = Instruction {
    //     program_id: crate::ID,
    //     accounts: vec![
    //         AccountMeta::new_readonly(authority.key(), false),
    //         AccountMeta::new(cleanup_queue.key(), true),
    //         AccountMeta::new(registry.key(), false),
    //     ],
    //     data: clockwork_crank::anchor::sighash("snapshot_close").into(),
    // };
    // clockwork_crank::cpi::queue_create(
    //     CpiContext::new_with_signer(
    //         clockwork_program.to_account_info(),
    //         clockwork_crank::cpi::accounts::QueueCreate {
    //             authority: authority.to_account_info(),
    //             payer: admin.to_account_info(),
    //             queue: cleanup_queue.to_account_info(),
    //             system_program: system_program.to_account_info(),
    //         },
    //         &[&[SEED_AUTHORITY, &[bump]]]
    //     ),
    //     LAMPORTS_PER_SOL,
    //     snapshot_close_ix.into(),
    //     "cleanup".into(),
    //     Trigger::Cron { schedule: "0 * * * * * *".into() }
    // )?;

    // Create task to close archived snapshot
    // clockwork_crank::cpi::task_new(
    //     CpiContext::new_with_signer(
    //         clockwork_program.to_account_info(),
    //         clockwork_crank::cpi::accounts::TaskNew {
    //             authority: authority.to_account_info(),
    //             payer: admin.to_account_info(),
    //             queue: cleanup_queue.to_account_info(),
    //             system_program: system_program.to_account_info(),
    //             task: cleanup_task.to_account_info(),
    //         },
    //         &[&[SEED_AUTHORITY, &[bump]]],
    //     ),
    //     vec![snapshot_close_ix.into()],
    // )?;


    Ok(())
}

