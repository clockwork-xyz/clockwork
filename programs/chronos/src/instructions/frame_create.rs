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
        constraint = timestamp % config.frame_interval == 0 @ ErrorCode::Unknown,
        constraint = timestamp > clock.unix_timestamp as u64 @ ErrorCode::Unknown
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

    #[account(mut)]
    pub list: AccountInfo<'info>,

    #[account(address = list_program::ID)]
    pub list_program: Program<'info, list_program::program::ListProgram>,

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
    let list = &ctx.accounts.list;
    let payer = &ctx.accounts.payer;
    let list_program = &ctx.accounts.list_program;
    let system_program = &ctx.accounts.system_program;

    // Initialize frame account.
    frame.timestamp = timestamp;
    frame.bump = frame_bump;

    // Create an list to lookup tasks by the time frame when they should be processed.
    list_program::cpi::create_list(
        CpiContext::new_with_signer(
            list_program.to_account_info(),
            list_program::cpi::accounts::CreateList {
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
