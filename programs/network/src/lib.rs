pub mod errors;
pub mod objects;

mod instructions;

use anchor_lang::prelude::*;
use clockwork_utils::*;
use instructions::*;
use objects::*;

declare_id!("HTKVtLQ4jSC1LPzbgji2mQcVKBTpUM72hDzsDWeXfrP3");

#[program]
pub mod network_program {
    use super::*;

    pub fn config_update(ctx: Context<ConfigUpdate>, settings: ConfigSettings) -> Result<()> {
        config_update::handler(ctx, settings)
    }

    pub fn delegation_create(ctx: Context<DelegationCreate>) -> Result<()> {
        delegation_create::handler(ctx)
    }

    pub fn delegation_deposit(ctx: Context<DelegationDeposit>, amount: u64) -> Result<()> {
        delegation_deposit::handler(ctx, amount)
    }

    pub fn delegation_stake(ctx: Context<DelegationStake>) -> Result<CrankResponse> {
        delegation_stake::handler(ctx)
    }

    pub fn delegation_withdraw(ctx: Context<DelegationWithdraw>, amount: u64) -> Result<()> {
        delegation_withdraw::handler(ctx, amount)
    }

    pub fn delegation_yield(ctx: Context<DelegationYield>, amount: u64) -> Result<()> {
        delegation_yield::handler(ctx, amount)
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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn pool_create(ctx: Context<PoolCreate>) -> Result<()> {
        pool_create::handler(ctx)
    }

    pub fn pool_rotate(ctx: Context<PoolRotate>) -> Result<()> {
        pool_rotate::handler(ctx)
    }

    pub fn pool_update(ctx: Context<PoolUpdate>, settings: PoolSettings) -> Result<()> {
        pool_update::handler(ctx, settings)
    }

    pub fn snapshot_delete(ctx: Context<SnapshotDelete>) -> Result<()> {
        snapshot_delete::handler(ctx)
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

    pub fn unstake_create(ctx: Context<UnstakeCreate>, amount: u64) -> Result<()> {
        unstake_create::handler(ctx, amount)
    }

    pub fn unstake_preprocess(ctx: Context<UnstakePreprocess>) -> Result<CrankResponse> {
        unstake_preprocess::handler(ctx)
    }

    pub fn unstake_process(ctx: Context<UnstakeProcess>) -> Result<CrankResponse> {
        unstake_process::handler(ctx)
    }

    pub fn worker_create(ctx: Context<WorkerCreate>) -> Result<()> {
        worker_create::handler(ctx)
    }

    pub fn worker_distribute_fees(ctx: Context<WorkerDistributeFees>) -> Result<CrankResponse> {
        worker_distribute_fees::handler(ctx)
    }

    pub fn worker_stake_delegations(ctx: Context<WorkerStakeDelegations>) -> Result<CrankResponse> {
        worker_stake_delegations::handler(ctx)
    }

    pub fn worker_update(ctx: Context<WorkerUpdate>, settings: WorkerSettings) -> Result<()> {
        worker_update::handler(ctx, settings)
    }
}
