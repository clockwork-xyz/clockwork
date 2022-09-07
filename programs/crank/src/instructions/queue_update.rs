use {
    crate::state::*,
    anchor_lang::{prelude::*, system_program::{transfer, Transfer}, solana_program::system_program},
};


#[derive(Accounts)]
#[instruction(kickoff_instruction: Option<InstructionData>, trigger: Option<Trigger>)]
pub struct QueueUpdate<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [
            SEED_QUEUE, 
            queue.authority.key().as_ref(),
            queue.id.as_bytes(),
        ],
        bump,
        has_one = authority,
    )]
    pub queue: Account<'info, Queue>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<QueueUpdate>, kickoff_instruction: Option<InstructionData>, trigger: Option<Trigger>) -> Result<()> {
    // Get accounts
    let authority = &ctx.accounts.authority;
    let queue = &mut ctx.accounts.queue;
    let system_program = &ctx.accounts.system_program;

    // If provided, update the queue's first instruction
    if let Some(kickoff_instruction) = kickoff_instruction {
        queue.kickoff_instruction = kickoff_instruction;
    }

    // If provided, update the queue's trigger and reset the exec context
    if let Some(trigger) = trigger {
        queue.trigger = trigger;
        queue.exec_context = None;
    }

    // Reallocate mem for the queue account
    queue.realloc()?;

    // If lamports are required to maintain rent-exemption, pay them
    let data_len = 8 + queue.try_to_vec()?.len();
    let minimum_rent = Rent::get().unwrap().minimum_balance(data_len);
    if minimum_rent > queue.to_account_info().lamports() {
        transfer(
            CpiContext::new(
                system_program.to_account_info(),
                Transfer {
                    from: authority.to_account_info(),
                    to: queue.to_account_info(),
                },
            ),
            minimum_rent
                .checked_sub(queue.to_account_info().lamports())
                .unwrap(),
        )?;
    }

    Ok(())
}