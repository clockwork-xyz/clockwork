use {
    crate::observers::network::PoolPositions,
    clockwork_client::{
        network::objects::{Pool, Registry, Snapshot, SnapshotFrame, Worker},
        Client as ClockworkClient,
    },
    solana_sdk::transaction::Transaction,
    std::sync::Arc,
    tokio::sync::RwLockReadGuard,
};

pub fn build_pool_rotation_tx<'a>(
    client: Arc<ClockworkClient>,
    r_pool_positions: RwLockReadGuard<'a, PoolPositions>,
    r_registry: RwLockReadGuard<'a, Registry>,
    r_snapshot: RwLockReadGuard<'a, Snapshot>,
    r_snapshot_frame: RwLockReadGuard<'a, Option<SnapshotFrame>>,
    worker_id: u64,
) -> Option<Transaction> {
    // Exit early if the rotator is not intialized
    if r_registry.nonce == 0 {
        return None;
    }

    // Exit early the snapshot has no stake
    if r_snapshot.total_stake == 0 {
        return None;
    }

    // Exit early if the worker is already in the pool.
    if r_pool_positions.queue_pool.current_position.is_some() {
        return None;
    }

    // Exit early if the snapshot frame is none or the worker has no delegated stake.
    if r_snapshot_frame.is_none() || r_snapshot_frame.clone().unwrap().stake_amount.eq(&0) {
        return None;
    }

    // Check if the rotation window is open for this worker.
    let is_rotation_window_open = match r_registry.nonce.checked_rem(r_snapshot.total_stake) {
        None => false,
        Some(sample) => {
            sample >= r_snapshot_frame.clone().unwrap().stake_offset
                && sample
                    < r_snapshot_frame
                        .clone()
                        .unwrap()
                        .stake_offset
                        .checked_add(r_snapshot_frame.clone().unwrap().stake_amount)
                        .unwrap()
        }
    };
    if !is_rotation_window_open {
        return None;
    }

    // Build rotation instruction to rotate the worker into pool 0.
    let snapshot_pubkey = Snapshot::pubkey(r_snapshot.id);
    let ix = clockwork_client::network::instruction::pool_rotate(
        Pool::pubkey(0),
        client.payer_pubkey(),
        snapshot_pubkey,
        SnapshotFrame::pubkey(snapshot_pubkey, worker_id),
        Worker::pubkey(worker_id),
    );

    // Drop read locks.
    drop(r_registry);
    drop(r_snapshot);
    drop(r_snapshot_frame);

    // Build and sign tx.
    let mut tx = Transaction::new_with_payer(&[ix.clone()], Some(&client.payer_pubkey()));
    tx.sign(&[client.payer()], client.get_latest_blockhash().unwrap());

    return Some(tx);
}
