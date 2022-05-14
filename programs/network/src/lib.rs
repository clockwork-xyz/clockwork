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

    pub fn snapshot_capture(ctx: Context<SnapshotCapture>) -> Result<ExecResponse> {
        snapshot_capture::handler(ctx)
    }

    pub fn snapshot_rotate(ctx: Context<SnapshotRotate>) -> Result<ExecResponse> {
        snapshot_rotate::handler(ctx)
    }

    pub fn snapshot_start(ctx: Context<SnapshotStart>) -> Result<ExecResponse> {
        snapshot_start::handler(ctx)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        stake::handler(ctx, amount)
    }
}
