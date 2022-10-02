use {
    crate::state::*,
    anchor_lang::{
        prelude::*, 
        solana_program::{instruction::Instruction, system_program}
    },
    anchor_spl::token::Mint,
    clockwork_queue_program::objects::{Queue, Trigger},
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

    #[account(address = clockwork_queue_program::ID)]
    pub clockwork_program: Program<'info, clockwork_queue_program::program::QueueProgram>,

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

    #[account(address = Queue::pubkey(authority.key(), "snapshot".into()))]
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
    config.init(admin.key(), mint.key())?;
    registry.init()?;
    rotator.init()?;

    // Setup the first snapshot
    registry.new_snapshot(snapshot)?;
    registry.rotate_snapshot(None, snapshot)?;

    // Create a queue to take snapshots of the registry
    let bump = *ctx.bumps.get("authority").unwrap();
    let snapshot_kickoff_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new_readonly(authority.key(), false),
            AccountMeta::new(registry.key(), false),
            AccountMeta::new_readonly(snapshot_queue.key(), true),
        ],
        data: clockwork_queue_program::utils::anchor_sighash("snapshot_kickoff").into(),
    };
    clockwork_queue_program::cpi::queue_create(
        CpiContext::new_with_signer(
            clockwork_program.to_account_info(),
            clockwork_queue_program::cpi::accounts::QueueCreate {
                authority: authority.to_account_info(),
                payer: admin.to_account_info(),
                queue: snapshot_queue.to_account_info(),
                system_program: system_program.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[bump]]]
        ),
        "snapshot".into(),
        snapshot_kickoff_ix.into(),
        Trigger::Cron { schedule: "0 * * * * * *".into(), skippable: true }
    )?;

    Ok(())
}

