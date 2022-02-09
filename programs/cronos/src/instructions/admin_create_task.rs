use {
    crate::{errors::*, state::*},
    anchor_lang::{prelude::*, solana_program::{system_program, sysvar}},
    std::mem::{size_of, size_of_val},
};


#[derive(Accounts)]
#[instruction(
    ix: InstructionData,
    exec_at: i64,
    stop_at: i64,
    recurr: i64,
    bump: u8
)]
pub struct AdminCreateTask<'info> {
    #[account(mut, address = config.admin)]
    pub admin: Signer<'info>,

    #[account(
        seeds = [SEED_AUTHORITY], 
        bump = authority.bump, 
        owner = crate::ID
    )]
    pub authority: Account<'info, Authority>,
    
    #[account(
        address = sysvar::clock::ID,
        constraint = exec_at >= clock.unix_timestamp - 60 @ ErrorCode::InvalidExecAtStale
    )]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        owner = crate::ID,
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [
            SEED_DAEMON, 
            daemon.owner.as_ref()
        ],
        bump = daemon.bump,
        constraint = daemon.owner == authority.key(),
        owner = crate::ID,
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    #[account(
        init,
        seeds = [
            SEED_TASK, 
            daemon.key().as_ref(),
            daemon.task_count.to_be_bytes().as_ref(),
        ],
        bump = bump,
        payer = admin,
        space = 32 + size_of::<Task>() + size_of_val(&ix), 
    )]
    pub task: Account<'info, Task>,
}

pub fn handler(
    ctx: Context<AdminCreateTask>, 
    ix: InstructionData,
    exec_at: i64,
    stop_at: i64,
    recurr: i64,
    bump: u8
) -> ProgramResult {
    // Get accounts.
    let daemon = &mut ctx.accounts.daemon;
    let task = &mut ctx.accounts.task;

    // Validate the scheduling chronology.
    require!(exec_at <= stop_at, ErrorCode::InvalidChronology);
    require!(recurr >= 0, ErrorCode::InvalidRecurrNegative);

    // Reject the instruction if it has other signers besides the daemon.
    for acc in ix.accounts.as_slice() {
        require!(
            !acc.is_signer || acc.pubkey == daemon.key(), 
            ErrorCode::InvalidSignatory
        );
    }

    // Initialize task account.
    task.daemon = daemon.key();
    task.id = daemon.task_count;
    task.ix = ix;
    task.status = TaskStatus::Pending;
    task.exec_at = exec_at;
    task.stop_at = stop_at;
    task.recurr = recurr;
    task.bump = bump;

    // Increment daemon task counter.
    daemon.task_count = daemon.task_count.checked_add(1).unwrap();

    Ok(())
}
