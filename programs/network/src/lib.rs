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

    pub fn delegation_create(ctx: Context<DelegationCreate>) -> Result<()> {
        delegation_create::handler(ctx)
    }

    pub fn delegation_request_unstake(
        ctx: Context<DelegationRequestUnstake>,
        amount: u64,
    ) -> Result<()> {
        delegation_request_unstake::handler(ctx, amount)
    }

    pub fn entry_close(ctx: Context<EntryClose>) -> Result<()> {
        snapshot_entry_close::handler(ctx)
    }

    pub fn entry_create(ctx: Context<EntryCreate>) -> Result<()> {
        snapshot_entry_create::handler(ctx)
    }

    pub fn epoch_start(ctx: Context<EpochStart>) -> Result<CrankResponse> {
        epoch_start::handler(ctx)
    }

    pub fn initialize<'info>(ctx: Context<'_, '_, '_, 'info, Initialize<'info>>) -> Result<()> {
        initialize::handler(ctx)
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

    pub fn snapshot_create(ctx: Context<SnapshotCreate>) -> Result<CrankResponse> {
        snapshot_create::handler(ctx)
    }

    pub fn snapshot_frame_create(ctx: Context<SnapshotFrameCreate>) -> Result<CrankResponse> {
        snapshot_frame_create::handler(ctx)
    }

    pub fn worker_register(ctx: Context<WorkerRegister>) -> Result<()> {
        worker_register::handler(ctx)
    }

    pub fn worker_update(ctx: Context<WorkerUpdate>, settings: WorkerSettings) -> Result<()> {
        worker_update::handler(ctx, settings)
    }
}
