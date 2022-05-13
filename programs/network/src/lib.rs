pub mod errors;
pub mod id;
pub mod pda;
pub mod state;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use cronos_scheduler::responses::ExecResponse;
use instructions::*;

#[program]
pub mod cronos_network {

    use super::*;

    pub fn initialize<'info>(ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn register<'info>(ctx: Context<'_, '_, '_, 'info, Register<'info>>) -> Result<()> {
        register::handler(ctx)
    }

    pub fn rotate_snapshot(ctx: Context<RotateSnapshot>) -> Result<ExecResponse> {
        rotate_snapshot::handler(ctx)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        stake::handler(ctx, amount)
    }

    pub fn start_snapshot(ctx: Context<StartSnapshot>) -> Result<ExecResponse> {
        start_snapshot::handler(ctx)
    }
}
