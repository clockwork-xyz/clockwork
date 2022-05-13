use {crate::state::*, anchor_lang::prelude::*, cronos_scheduler::state::Queue, std::mem::size_of};

#[derive(Accounts)]
pub struct StartSnapshot<'info> {
    #[account(seeds = [SEED_CONFIG], bump)]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(signer)]
    pub queue: Account<'info, Queue>,

    #[account(mut, seeds = [SEED_REGISTRY], bump)]
    pub registry: Account<'info, Registry>,

    #[account(
        init,
        seeds = [
            SEED_SNAPSHOT,
            registry.snapshot_count.to_be_bytes().as_ref()
        ],
        bump,
        space = 8 + size_of::<Snapshot>(),
        payer = payer
    )]
    pub snapshot: Account<'info, Snapshot>,

    #[account()]
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<StartSnapshot>) -> Result<()> {
    msg!("Starting snapshot!");

    let registry = &mut ctx.accounts.registry;
    let snapshot = &mut ctx.accounts.snapshot;

    registry.new_snapshot(snapshot)?;

    Ok(())
}
