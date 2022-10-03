pub mod errors;
pub mod objects;

mod instructions;

use anchor_lang::prelude::*;
use clockwork_queue_program::objects::CrankResponse;
use instructions::*;
use objects::*;

declare_id!("7uTtwxSLrUs6rTWzqYnvoQ8JnmcWT9WPDdoN1ojmvD6t");

#[program]
pub mod network_program {
    use super::*;

    pub fn config_update(ctx: Context<ConfigUpdate>, settings: ConfigSettings) -> Result<()> {
        config_update::handler(ctx, settings)
    }

    pub fn entry_close(ctx: Context<EntryClose>) -> Result<CrankResponse> {
        entry_close::handler(ctx)
    }

    pub fn entry_create(ctx: Context<EntryCreate>) -> Result<CrankResponse> {
        entry_create::handler(ctx)
    }

    pub fn initialize<'info>(ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn node_register(ctx: Context<NodeRegister>) -> Result<()> {
        node_register::handler(ctx)
    }

    pub fn node_stake(ctx: Context<NodeStake>, amount: u64) -> Result<()> {
        node_stake::handler(ctx, amount)
    }

    pub fn node_update(ctx: Context<NodeUpdate>, settings: NodeSettings) -> Result<()> {
        node_update::handler(ctx, settings)
    }

    pub fn node_unstake(ctx: Context<NodeUnstake>, amount: u64) -> Result<()> {
        node_unstake::handler(ctx, amount)
    }

    pub fn pool_create(ctx: Context<PoolCreate>, name: String, size: usize) -> Result<()> {
        pool_create::handler(ctx, name, size)
    }

    pub fn pools_rotate<'info>(ctx: Context<'_, '_, '_, 'info, PoolsRotate<'info>>) -> Result<()> {
        pools_rotate::handler(ctx)
    }

    pub fn snapshot_close(ctx: Context<SnapshotClose>) -> Result<CrankResponse> {
        snapshot_close::handler(ctx)
    }

    pub fn snapshot_create(ctx: Context<SnapshotCreate>) -> Result<CrankResponse> {
        snapshot_create::handler(ctx)
    }

    pub fn snapshot_kickoff(ctx: Context<SnapshotKickoff>) -> Result<CrankResponse> {
        snapshot_kickoff::handler(ctx)
    }

    pub fn snapshot_pause(ctx: Context<SnapshotPause>) -> Result<()> {
        snapshot_pause::handler(ctx)
    }

    pub fn snapshot_resume(ctx: Context<SnapshotResume>) -> Result<()> {
        snapshot_resume::handler(ctx)
    }

    pub fn snapshot_rotate(ctx: Context<SnapshotRotate>) -> Result<CrankResponse> {
        snapshot_rotate::handler(ctx)
    }
}
