pub mod errors;
pub mod id;
pub mod state;

mod instructions;

pub use id::ID;

use anchor_lang::prelude::*;
use clockwork_crank::state::CrankResponse;
use instructions::*;

#[program]
pub mod clockwork_network {

    use super::*;

    pub fn entry_close(ctx: Context<EntryClose>) -> Result<CrankResponse> {
        entry_close::handler(ctx)
    }

    pub fn entry_create(ctx: Context<EntryCreate>) -> Result<CrankResponse> {
        entry_create::handler(ctx)
    }

    pub fn initialize<'info>(ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn rotator_turn(ctx: Context<RotatorTurn>) -> Result<()> {
        rotator_turn::handler(ctx)
    }

    pub fn node_register(ctx: Context<NodeRegister>) -> Result<()> {
        node_register::handler(ctx)
    }

    pub fn node_stake(ctx: Context<NodeStake>, amount: u64) -> Result<()> {
        node_stake::handler(ctx, amount)
    }

    pub fn snapshot_close(ctx: Context<SnapshotClose>) -> Result<CrankResponse> {
        snapshot_close::handler(ctx)
    }

    pub fn snapshot_create(ctx: Context<SnapshotCreate>) -> Result<CrankResponse> {
        snapshot_create::handler(ctx)
    }

    pub fn snapshot_queue_kickoff(ctx: Context<SnapshotQueueKickoff>) -> Result<CrankResponse> {
        snapshot_queue_kickoff::handler(ctx)
    }

    pub fn snapshot_rotate(ctx: Context<SnapshotRotate>) -> Result<CrankResponse> {
        snapshot_rotate::handler(ctx)
    }
}
