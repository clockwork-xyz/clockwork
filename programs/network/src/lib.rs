//! This program orchestrates a Clockwork worker network deployed across a Solana cluster.
//! It implements a PoS protocol that allows workers to rotate into "pools" proportionately to
//! the amount of stake delgated to them. It also provides accounts for workers to collect fees
//! and distribute those fees to delegators.

pub mod errors;
pub mod state;

mod instructions;
mod jobs;

use anchor_lang::prelude::*;
use clockwork_utils::thread::*;
use instructions::*;
use jobs::*;
use state::*;

declare_id!("F8dKseqmBoAkHx3c58Lmb9TgJv5qeTf3BbtZZSEzYvUa");

#[program]
pub mod network_program {
    pub use super::*;

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

    pub fn delegation_withdraw(ctx: Context<DelegationWithdraw>, amount: u64) -> Result<()> {
        delegation_withdraw::handler(ctx, amount)
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

    pub fn registry_nonce_hash(ctx: Context<RegistryNonceHash>) -> Result<ThreadResponse> {
        registry_nonce_hash::handler(ctx)
    }

    pub fn registry_unlock(ctx: Context<RegistryUnlock>) -> Result<()> {
        registry_unlock::handler(ctx)
    }

    pub fn unstake_create(ctx: Context<UnstakeCreate>, amount: u64) -> Result<()> {
        unstake_create::handler(ctx, amount)
    }

    pub fn worker_claim(ctx: Context<WorkerClaim>, amount: u64) -> Result<()> {
        worker_claim::handler(ctx, amount)
    }

    pub fn worker_create(ctx: Context<WorkerCreate>) -> Result<()> {
        worker_create::handler(ctx)
    }

    pub fn worker_update(ctx: Context<WorkerUpdate>, settings: WorkerSettings) -> Result<()> {
        worker_update::handler(ctx, settings)
    }

    // DistributeFees job

    pub fn distribute_fees_job(ctx: Context<DistributeFeesJob>) -> Result<ThreadResponse> {
        jobs::distribute_fees::job::handler(ctx)
    }

    pub fn distribute_fees_process_entry(
        ctx: Context<DistributeFeesProcessEntry>,
    ) -> Result<ThreadResponse> {
        jobs::distribute_fees::process_entry::handler(ctx)
    }

    pub fn distribute_fees_process_frame(
        ctx: Context<DistributeFeesProcessFrame>,
    ) -> Result<ThreadResponse> {
        jobs::distribute_fees::process_frame::handler(ctx)
    }

    pub fn distribute_fees_process_snapshot(
        ctx: Context<DistributeFeesProcessSnapshot>,
    ) -> Result<ThreadResponse> {
        jobs::distribute_fees::process_snapshot::handler(ctx)
    }

    // StakeDelegations job

    pub fn stake_delegations_job(ctx: Context<StakeDelegationsJob>) -> Result<ThreadResponse> {
        jobs::stake_delegations::job::handler(ctx)
    }

    pub fn stake_delegations_process_worker(
        ctx: Context<StakeDelegationsProcessWorker>,
    ) -> Result<ThreadResponse> {
        jobs::stake_delegations::process_worker::handler(ctx)
    }

    pub fn stake_delegations_process_delegation(
        ctx: Context<StakeDelegationsProcessDelegation>,
    ) -> Result<ThreadResponse> {
        jobs::stake_delegations::process_delegation::handler(ctx)
    }

    // TakeSnapshot job

    pub fn take_snapshot_job(ctx: Context<TakeSnapshotJob>) -> Result<ThreadResponse> {
        jobs::take_snapshot::job::handler(ctx)
    }

    pub fn take_snapshot_create_entry(
        ctx: Context<TakeSnapshotCreateEntry>,
    ) -> Result<ThreadResponse> {
        jobs::take_snapshot::create_entry::handler(ctx)
    }

    pub fn take_snapshot_create_frame(
        ctx: Context<TakeSnapshotCreateFrame>,
    ) -> Result<ThreadResponse> {
        jobs::take_snapshot::create_frame::handler(ctx)
    }

    pub fn take_snapshot_create_snapshot(
        ctx: Context<TakeSnapshotCreateSnapshot>,
    ) -> Result<ThreadResponse> {
        jobs::take_snapshot::create_snapshot::handler(ctx)
    }

    // IncrementEpoch job

    pub fn increment_epoch(ctx: Context<EpochCutover>) -> Result<ThreadResponse> {
        jobs::increment_epoch::job::handler(ctx)
    }

    // Delete snapshot

    pub fn delete_snapshot_job(ctx: Context<DeleteSnapshotJob>) -> Result<ThreadResponse> {
        jobs::delete_snapshot::job::handler(ctx)
    }

    pub fn delete_snapshot_process_snapshot(
        ctx: Context<DeleteSnapshotProcessSnapshot>,
    ) -> Result<ThreadResponse> {
        jobs::delete_snapshot::process_snapshot::handler(ctx)
    }

    pub fn delete_snapshot_process_frame(
        ctx: Context<DeleteSnapshotProcessFrame>,
    ) -> Result<ThreadResponse> {
        jobs::delete_snapshot::process_frame::handler(ctx)
    }

    pub fn delete_snapshot_process_entry(
        ctx: Context<DeleteSnapshotProcessEntry>,
    ) -> Result<ThreadResponse> {
        jobs::delete_snapshot::process_entry::handler(ctx)
    }

    // ProcessUnstakes job

    pub fn process_unstakes_job(ctx: Context<ProcessUnstakesJob>) -> Result<ThreadResponse> {
        jobs::process_unstakes::job::handler(ctx)
    }

    pub fn unstake_preprocess(ctx: Context<UnstakePreprocess>) -> Result<ThreadResponse> {
        jobs::process_unstakes::unstake_preprocess::handler(ctx)
    }

    pub fn unstake_process(ctx: Context<UnstakeProcess>) -> Result<ThreadResponse> {
        jobs::process_unstakes::unstake_process::handler(ctx)
    }
}
