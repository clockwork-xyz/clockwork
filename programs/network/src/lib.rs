pub mod errors;
pub mod objects;

mod instructions;

use anchor_lang::prelude::*;
use clockwork_utils::*;
use instructions::*;
use objects::*;

declare_id!("7PVusEAWWF55ExBBpwQdQfPCaHMUXXbHAP2iSVtNeAvP");

#[program]
pub mod network_program {
    use super::*;

    pub fn config_update(ctx: Context<ConfigUpdate>, settings: ConfigSettings) -> Result<()> {
        config_update::handler(ctx, settings)
    }

    pub fn entry_close(ctx: Context<EntryClose>) -> Result<()> {
        entry_close::handler(ctx)
    }

    pub fn entry_create(ctx: Context<EntryCreate>) -> Result<()> {
        entry_create::handler(ctx)
    }

    pub fn epoch_start(ctx: Context<EpochStart>) -> Result<CrankResponse> {
        epoch_start::handler(ctx)
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

    pub fn pool_rotate(ctx: Context<PoolRotate>) -> Result<()> {
        pool_rotate::handler(ctx)
    }

    pub fn pool_update(ctx: Context<PoolUpdate>, settings: PoolSettings) -> Result<()> {
        pool_update::handler(ctx, settings)
    }

    pub fn snapshot_close(ctx: Context<SnapshotClose>) -> Result<()> {
        snapshot_close::handler(ctx)
    }

    pub fn snapshot_create(ctx: Context<SnapshotCreate>) -> Result<()> {
        snapshot_create::handler(ctx)
    }
}
