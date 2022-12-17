use {
    crate::observers::network::PoolPosition,
    clockwork_client::{
        network::state::{Pool, Registry, Snapshot, SnapshotFrame, Worker},
        Client as ClockworkClient,
    },
    solana_sdk::transaction::Transaction,
    std::sync::Arc,
};

pub fn build_pool_rotation_tx<'a>(
    client: Arc<ClockworkClient>,
    pool_position: PoolPosition,
    registry: Registry,
    snapshot: &Snapshot,
    snapshot_frame: &SnapshotFrame,
    worker_id: u64,
) -> Option<Transaction> {
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
    let ix = clockwork_client::network::instruction::pool_rotate(
        Pool::pubkey(0),
        client.payer_pubkey(),
        snapshot_pubkey,
        SnapshotFrame::pubkey(snapshot_pubkey, worker_id),
        Worker::pubkey(worker_id),
    );

    // Build and sign tx.
    let mut tx = Transaction::new_with_payer(&[ix.clone()], Some(&client.payer_pubkey()));
    tx.sign(&[client.payer()], client.get_latest_blockhash().unwrap());

    return Some(tx);
}
