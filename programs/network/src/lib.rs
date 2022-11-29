//! This program orchestrates a Clockwork worker network deployed across a Solana cluster.
//! It implements a PoS protocol that allows workers to rotate into "pools" proportionately to
//! the amount of stake delgated to them. It also provides accounts for workers to collect fees
//! and distribute those fees to delegators.

pub mod errors;
pub mod state;

mod instructions;

use anchor_lang::prelude::*;
use clockwork_utils::*;
use instructions::*;
use state::*;

declare_id!("F8dKseqmBoAkHx3c58Lmb9TgJv5qeTf3BbtZZSEzYvUa");

#[program]
pub mod network_program {
    use super::*;

    pub fn config_update(ctx: Context<ConfigUpdate>, settings: ConfigSettings) -> Result<()> {
        config_update::handler(ctx, settings)
    }

    pub fn delegation_claim(ctx: Context<DelegationClaim>, amount: u64) -> Result<()> {
        delegation_claim::handler(ctx, amount)
    }

    pub fn delegation_create(ctx: Context<DelegationCreate>) -> Result<()> {
        delegation_create::handler(ctx)
    }

    pub fn delegation_deposit(ctx: Context<DelegationDeposit>, amount: u64) -> Result<()> {
        delegation_deposit::handler(ctx, amount)
    }

    pub fn delegation_stake(ctx: Context<DelegationStake>) -> Result<ThreadResponse> {
        delegation_stake::handler(ctx)
    }

    pub fn delegation_withdraw(ctx: Context<DelegationWithdraw>, amount: u64) -> Result<()> {
        delegation_withdraw::handler(ctx, amount)
    }

    pub fn fee_distribute(ctx: Context<FeeDistribute>) -> Result<ThreadResponse> {
        fee_distribute::handler(ctx)
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn penalty_claim(ctx: Context<PenaltyClaim>) -> Result<()> {
        penalty_claim::handler(ctx)
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

    pub fn registry_epoch_cutover(ctx: Context<RegistryEpochCutover>) -> Result<ThreadResponse> {
        registry_epoch_cutover::handler(ctx)
    }

    pub fn registry_epoch_kickoff(ctx: Context<RegistryEpochKickoff>) -> Result<ThreadResponse> {
        registry_epoch_kickoff::handler(ctx)
    }

    pub fn registry_nonce_hash(ctx: Context<RegistryNonceHash>) -> Result<ThreadResponse> {
        registry_nonce_hash::handler(ctx)
    }

    pub fn registry_unlock(ctx: Context<RegistryUnlock>) -> Result<()> {
        registry_unlock::handler(ctx)
    }

    pub fn snapshot_delete(ctx: Context<SnapshotDelete>) -> Result<ThreadResponse> {
        snapshot_delete::handler(ctx)
    }

    pub fn snapshot_create(ctx: Context<SnapshotCreate>) -> Result<ThreadResponse> {
        snapshot_create::handler(ctx)
    }

    pub fn snapshot_entry_create(ctx: Context<SnapshotEntryCreate>) -> Result<ThreadResponse> {
        snapshot_entry_create::handler(ctx)
    }

    pub fn snapshot_entry_delete(ctx: Context<SnapshotEntryDelete>) -> Result<ThreadResponse> {
        snapshot_entry_delete::handler(ctx)
    }

    pub fn snapshot_frame_create(ctx: Context<SnapshotFrameCreate>) -> Result<ThreadResponse> {
        snapshot_frame_create::handler(ctx)
    }

    pub fn snapshot_frame_delete(ctx: Context<SnapshotFrameDelete>) -> Result<ThreadResponse> {
        snapshot_frame_delete::handler(ctx)
    }

    pub fn unstake_create(ctx: Context<UnstakeCreate>, amount: u64) -> Result<()> {
        unstake_create::handler(ctx, amount)
    }

    pub fn unstake_preprocess(ctx: Context<UnstakePreprocess>) -> Result<ThreadResponse> {
        unstake_preprocess::handler(ctx)
    }

    pub fn unstake_process(ctx: Context<UnstakeProcess>) -> Result<ThreadResponse> {
        unstake_process::handler(ctx)
    }

    pub fn worker_claim(ctx: Context<WorkerClaim>, amount: u64) -> Result<()> {
        worker_claim::handler(ctx, amount)
    }

    pub fn worker_create(ctx: Context<WorkerCreate>) -> Result<()> {
        worker_create::handler(ctx)
    }

    pub fn worker_fees_distribute(ctx: Context<WorkerDistributeFees>) -> Result<ThreadResponse> {
        worker_fees_distribute::handler(ctx)
    }

    pub fn worker_delegations_stake(
        ctx: Context<WorkerStakeDelegations>,
    ) -> Result<ThreadResponse> {
        worker_delegations_stake::handler(ctx)
    }

    pub fn worker_update(ctx: Context<WorkerUpdate>, settings: WorkerSettings) -> Result<()> {
        worker_update::handler(ctx, settings)
    }
}
