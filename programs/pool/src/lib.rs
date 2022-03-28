pub mod errors;
pub mod pda;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use instructions::*;
// use state::*;

declare_id!("EKpcZk331JDfGaetHvU1LuUmDCh4Pv3392sfNZYt8gbQ");

#[program]
pub mod cronos_registry {
    use crate::instructions::Initialize;

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        config_bump: u8,
        pool_bump: u8,
        registry_bump: u8,
        snapshot_bump: u8
    ) -> Result<()> {
        initialize::handler(
            ctx,
            config_bump,
            pool_bump,
            registry_bump,
            snapshot_bump
        )
    }
    
}
