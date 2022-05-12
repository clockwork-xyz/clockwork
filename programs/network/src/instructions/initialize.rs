use {
    crate::state::*,
    anchor_lang::{
        prelude::*, 
        solana_program::{instruction::Instruction, system_program, sysvar}
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

    #[account(address = sysvar::clock::ID)]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        init,
        seeds = [SEED_CONFIG],
        bump,
        payer = admin,
        space = 8 + size_of::<Config>(),
    )]
    pub config: Account<'info, Config>,

    #[account()]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        seeds = [SEED_POOL],
        bump,
        payer = admin,
        space = 8 + size_of::<Pool>(),
    )]
    pub pool: Account<'info, Pool>,

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
    let clock = &ctx.accounts.clock;
    let config = &mut ctx.accounts.config;
    let mint = &ctx.accounts.mint;
    let pool = &mut ctx.accounts.pool;
    let registry = &mut ctx.accounts.registry;
    let scheduler_program = &ctx.accounts.scheduler_program;
    let snapshot = &mut ctx.accounts.snapshot;
    let system_program = &ctx.accounts.system_program;

    // Get remaining accounts
    let action = ctx.remaining_accounts.get(0).unwrap();
    let fee = ctx.remaining_accounts.get(1).unwrap();
    let queue = ctx.remaining_accounts.get(2).unwrap();
    let task = ctx.remaining_accounts.get(3).unwrap();
    
    // Get bumps
    let authority_bump = *ctx.bumps.get("authority").unwrap();

    // Initialize accounts
    authority.new(queue.key())?;
    config.new(admin.key(),  mint.key())?;
    pool.new()?;
    registry.new()?;
    registry.new_snapshot(snapshot)?;
    registry.rotate_snapshot(clock, None, snapshot)?;

    // Create a queue
    cronos_scheduler::cpi::queue_new(
        CpiContext::new_with_signer(
            scheduler_program.to_account_info(), 
            cronos_scheduler::cpi::accounts::QueueNew {
                fee: fee.to_account_info(),
                owner: authority.to_account_info(),
                payer: admin.to_account_info(),
                queue: queue.to_account_info(),
                system_program: system_program.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[authority_bump]]]
        )
    )?;

    // Create a task to collect snapshots
    cronos_scheduler::cpi::task_new(
        CpiContext::new_with_signer(
            scheduler_program.to_account_info(),
            cronos_scheduler::cpi::accounts::TaskNew {
                clock: clock.to_account_info(),
                owner: authority.to_account_info(),
                payer: admin.to_account_info(),
                queue: queue.to_account_info(),
                system_program: system_program.to_account_info(),
                task: task.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[authority_bump]]]
        ), 
        "0/20 * * * * * *".into()
    )?;

    // Create an action to start the snapshot
    let start_snapshot_ix = Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta {
                pubkey: config.key(),
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: queue.key(),
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: cronos_scheduler::delegate::ID,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: registry.key(),
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: Snapshot::pda(1).0,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: system_program.key(),
                is_signer: false,
                is_writable: false,
            }
        ],
        data: sighash("global", "start_snapshot").into(),
    };
    cronos_scheduler::cpi::action_new(
        CpiContext::new_with_signer(
            scheduler_program.to_account_info(),
            cronos_scheduler::cpi::accounts::ActionNew {
                action: action.to_account_info(),
                owner: authority.to_account_info(),
                payer: admin.to_account_info(),
                queue: queue.to_account_info(),
                system_program: system_program.to_account_info(),
                task: task.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[authority_bump]]],
        ),
        vec![start_snapshot_ix.into()],
    )?;

    Ok(())
}

pub fn sighash(namespace: &str, name: &str) -> [u8; 8] {
    let preimage = format!("{}:{}", namespace, name);
    let mut sighash = [0u8; 8];
    sighash.copy_from_slice(
        &anchor_lang::solana_program::hash::hash(preimage.as_bytes()).to_bytes()
            [..8],
    );
    sighash
}
