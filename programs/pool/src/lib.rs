pub mod errors;
pub mod id;
pub mod pda;
pub mod state;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use instructions::*;

#[program]
pub mod cronos_pool {
    use super::*;

    pub fn cycle(ctx: Context<Cycle>, delegate: Pubkey) -> Result<()> {
        cycle::handler(ctx, delegate)
    }

    pub fn initialize(ctx: Context<Initialize>, cycler: Pubkey) -> Result<()> {
        initialize::handler(ctx, cycler)
    }
}
