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

    pub fn delegation_lock(ctx: Context<DelegationLock>) -> Result<CrankResponse> {
        delegation_lock::handler(ctx)
    }

    pub fn epoch_create(ctx: Context<EpochCreate>) -> Result<CrankResponse> {
        epoch_create::handler(ctx)
    }

    pub fn epoch_cutover(ctx: Context<EpochCutover>) -> Result<CrankResponse> {
        epoch_cutover::handler(ctx)
    }

    pub fn epoch_kickoff(ctx: Context<EpochKickoff>) -> Result<CrankResponse> {
        epoch_kickoff::handler(ctx)
    }

    pub fn fee_distribute(ctx: Context<FeeDistribute>) -> Result<CrankResponse> {
        fee_distribute::handler(ctx)
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

    pub fn snapshot_entry_create(ctx: Context<SnapshotEntryCreate>) -> Result<CrankResponse> {
        snapshot_entry_create::handler(ctx)
    }

    pub fn snapshot_frame_create(ctx: Context<SnapshotFrameCreate>) -> Result<CrankResponse> {
        snapshot_frame_create::handler(ctx)
    }

    pub fn worker_distribute_fees(ctx: Context<WorkerDistributeFees>) -> Result<CrankResponse> {
        worker_distribute_fees::handler(ctx)
    }

    pub fn worker_lock_delegations(ctx: Context<WorkerLockDelegations>) -> Result<CrankResponse> {
        worker_lock_delegations::handler(ctx)
    }

    pub fn worker_register(ctx: Context<WorkerRegister>) -> Result<()> {
        worker_register::handler(ctx)
    }

    pub fn worker_update(ctx: Context<WorkerUpdate>, settings: WorkerSettings) -> Result<()> {
        worker_update::handler(ctx, settings)
    }
}
