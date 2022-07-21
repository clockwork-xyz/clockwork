pub mod errors;
pub mod id;
pub mod state;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use cronos_scheduler::responses::TaskResponse;
use instructions::*;

#[program]
pub mod cronos_network {

    use super::*;

    pub fn entry_close(ctx: Context<EntryClose>) -> Result<TaskResponse> {
        entry_close::handler(ctx)
    }

    pub fn initialize<'info>(ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn rotator_turn(ctx: Context<RotatorTurn>) -> Result<()> {
        rotator_turn::handler(ctx)
    }

    pub fn node_register<'info>(
        ctx: Context<'_, '_, '_, 'info, NodeRegister<'info>>,
    ) -> Result<()> {
        node_register::handler(ctx)
    }

    pub fn node_stake(ctx: Context<NodeStake>, amount: u64) -> Result<()> {
        node_stake::handler(ctx, amount)
    }

    pub fn snapshot_capture(ctx: Context<SnapshotCapture>) -> Result<TaskResponse> {
        snapshot_capture::handler(ctx)
    }

    pub fn snapshot_close(ctx: Context<SnapshotClose>) -> Result<TaskResponse> {
        snapshot_close::handler(ctx)
    }

    pub fn snapshot_rotate(ctx: Context<SnapshotRotate>) -> Result<TaskResponse> {
        snapshot_rotate::handler(ctx)
    }

    pub fn snapshot_start(ctx: Context<SnapshotStart>) -> Result<TaskResponse> {
        snapshot_start::handler(ctx)
    }
}
