use {
    crate::{errors::*, state::*},
    anchor_lang::{prelude::*, solana_program::system_program},
    std::mem::size_of,
};

#[derive(Accounts)]
#[instruction(
    timestamp: u64,
    frame_bump: u8,
    list_bump: u8,  
)]
pub struct WindowCreate<'info> {
    #[account(
        mut, 
        seeds = [SEED_AUTHORITY], 
        bump = authority.bump,
        owner = crate::ID,
    )]
    pub authority: Account<'info, Authority>,

    #[account(
        constraint = timestamp % config.frame_interval == 0 @ ErrorCode::InvalidTimestamp,
    )]
    pub clock: Sysvar<'info, Clock>,

    #[account(
        seeds = [SEED_CONFIG],
        bump = config.bump,
        owner = crate::ID
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        seeds = [
            SEED_FRAME,
            timestamp.to_be_bytes().as_ref()
        ],
        bump = frame_bump,
        payer = payer,
        space = 8 + size_of::<Frame>(),
    )]
    pub frame: Account<'info, Frame>,

    #[account(address = cronos_indexer::ID)]
    pub indexer_program: Program<'info, cronos_indexer::program::Indexer>,

    #[account(mut)]
    pub list: AccountInfo<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<WindowCreate>,
    timestamp: u64,
    frame_bump: u8,
    list_bump: u8,
) -> ProgramResult {
    // Get accounts.
    let authority = &ctx.accounts.authority;
    let frame = &mut ctx.accounts.frame;
    let indexer_program = &ctx.accounts.indexer_program;
    let list = &ctx.accounts.list;
    let payer = &ctx.accounts.payer;
    let system_program = &ctx.accounts.system_program;

    // Initialize frame account.
    frame.timestamp = timestamp;
    frame.bump = frame_bump;

    // Create an list to lookup tasks by the time frame when they should be processed.
    cronos_indexer::cpi::create_list(
        CpiContext::new_with_signer(
            indexer_program.to_account_info(),
            cronos_indexer::cpi::accounts::CreateList {
                list: list.to_account_info(),
                owner: authority.to_account_info(),
                payer: payer.to_account_info(),
                namespace: frame.to_account_info(),
                system_program: system_program.to_account_info(),
            },
            &[&[SEED_AUTHORITY, &[authority.bump]]],
        ),
        list_bump,
    )
}
