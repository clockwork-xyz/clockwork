use std::sync::Arc;

use anchor_lang::{
    solana_program::instruction::Instruction,
    InstructionData, ToAccountMetas
};
use clockwork_network_program::state::{Config, Pool, Registry, Snapshot, SnapshotFrame, Worker};
use log::info;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::pool_position::PoolPosition;

pub async fn build_pool_rotation_tx<'a>(
    client: Arc<RpcClient>,
    keypair: &Keypair,
    pool_position: PoolPosition,
    registry: Registry,
    snapshot: Snapshot,
    snapshot_frame: SnapshotFrame,
    worker_id: u64,
) -> Option<Transaction> {
    info!("nonce: {:?} total_stake: {:?} current_position: {:?} stake_offset: {:?} stake_amount: {:?}",
        registry.nonce.checked_rem(snapshot.total_stake),
        snapshot.total_stake,
        pool_position.current_position,
        snapshot_frame.stake_offset,
        snapshot_frame.stake_amount,
    );

    // Exit early if the rotator is not intialized
    if registry.nonce == 0 {
        return None;
    }

    // Exit early the snapshot has no stake
    if snapshot.total_stake == 0 {
        return None;
    }

    // Exit early if the worker is already in the pool.
    if pool_position.current_position.is_some() {
        return None;
    }

    // Exit early if the snapshot frame is none or the worker has no delegated stake.
    if snapshot_frame.stake_amount.eq(&0) {
        return None;
    }

    // Check if the rotation window is open for this worker.
    let is_rotation_window_open = match registry.nonce.checked_rem(snapshot.total_stake) {
        None => false,
        Some(sample) => {
            sample >= snapshot_frame.stake_offset
                && sample
                    < snapshot_frame
                        .stake_offset
                        .checked_add(snapshot_frame.stake_amount)
                        .unwrap()
        }
    };
    if !is_rotation_window_open {
        return None;
    }

    // Build rotation instruction to rotate the worker into pool 0.
    let snapshot_pubkey = Snapshot::pubkey(snapshot.id);
    let ix = Instruction {
        program_id: clockwork_network_program::ID,
        accounts: clockwork_network_program::accounts::PoolRotate {
            config: Config::pubkey(),
            pool: Pool::pubkey(0),
            registry: Registry::pubkey(),
            signatory: keypair.pubkey(),
            snapshot: snapshot_pubkey,
            snapshot_frame: SnapshotFrame::pubkey(snapshot_pubkey, worker_id),
            worker: Worker::pubkey(worker_id),
        }.to_account_metas(Some(false)),
        data: clockwork_network_program::instruction::PoolRotate {}.data(),
    };

    // Build and sign tx.
    let mut tx = Transaction::new_with_payer(&[ix.clone()], Some(&keypair.pubkey()));
    tx.sign(&[keypair], client.get_latest_blockhash().await.unwrap());
    return Some(tx);
}
