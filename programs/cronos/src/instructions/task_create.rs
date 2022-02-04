use {
    crate::{state::*, errors::*},
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(
    instruction_data: InstructionData,
    execute_at: u64, 
    repeat_every: u64,
    repeat_until: u64,
    task_bump: u8,
    task_element_bump: u8,
)]
pub struct TaskCreate<'info> {
    #[account(
        mut,
        seeds = [SEED_AUTHORITY],
        bump = authority.bump,
        owner = crate::ID
    )]
    pub authority: Account<'info, Authority>,

    #[account(
        mut,
        seeds = [
            SEED_DAEMON, 
            daemon.owner.as_ref()
        ],
        bump = daemon.bump,
        has_one = owner,
        owner = crate::ID
    )]
    pub daemon: Account<'info, Daemon>,

    #[account(
        seeds = [
            SEED_FRAME, 
            frame.timestamp.to_be_bytes().as_ref()
        ],
        bump = frame.bump,
        constraint = frame.timestamp == execute_at,
        owner = crate::ID
    )]
    pub frame: Account<'info, Frame>,

    #[account(address = cronos_indexer::ID)]
    pub indexer_program: Program<'info, cronos_indexer::program::Indexer>,

    #[account(
        init,
        seeds = [
            SEED_TASK, 
            daemon.key().as_ref(),
            daemon.task_count.to_be_bytes().as_ref(),
        ],
        bump = task_bump,
        payer = owner,
        space = 8 + size_of::<Task>() + std::mem::size_of_val(&instruction_data),
    )]
    pub task: Account<'info, Task>,

    #[account(mut)]
    pub task_element: AccountInfo<'info>,

    #[account(
        mut,
        constraint = task_list.namespace == frame.key(),
        owner = cronos_indexer::ID
    )]
    pub task_list: Account<'info, cronos_indexer::state::List>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<TaskCreate>,
    instruction_data: InstructionData,
    execute_at: u64, 
    repeat_every: u64,
    repeat_until: u64,
    task_bump: u8,
    task_element_bump: u8,
) -> ProgramResult {
    // Get accounts.
    let authority = &ctx.accounts.authority;
    let daemon = &mut ctx.accounts.daemon;
    let indexer_program = &ctx.accounts.indexer_program;
    let owner = &ctx.accounts.owner;
    let system_program = &ctx.accounts.system_program;
    let task = &mut ctx.accounts.task;
    let task_element = &ctx.accounts.task_element;
    let task_list = &ctx.accounts.task_list;

    // Validate the daemon is the only required signer on the instruction.
    // If the instruction has other required signers, we should just fail now.
    for acc in instruction_data.keys.as_slice() {
        require!(
            !acc.is_signer || acc.pubkey == daemon.key(), 
            ErrorCode::InvalidSignatory
        );
    }

    // Initialize task account.
    task.daemon = daemon.key();
    task.id = daemon.task_count;
    task.instruction_data = instruction_data;
    task.status = TaskStatus::Pending;
    task.execute_at = execute_at;
    task.repeat_every = repeat_every;
    task.repeat_until = repeat_until;
    task.bump = task_bump;

    // Increment daemon task counter.
    daemon.task_count = daemon.task_count.checked_add(1).unwrap();

    // Add task to list for execution in the given timeframe.
    cronos_indexer::cpi::push_element(
        CpiContext::new_with_signer(
            indexer_program.to_account_info(), 
            cronos_indexer::cpi::accounts::PushElement {
                list: task_list.to_account_info(),
                element: task_element.to_account_info(),
                owner: authority.to_account_info(),
                payer: owner.to_account_info(),
                system_program: system_program.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[authority.bump]]]
        ),
        task.key(),
        task_element_bump, 
    )?;

    return Ok(());
}
