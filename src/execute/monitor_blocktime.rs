use {
    solana_client::pubsub_client::PubsubClient,
    std::{
        sync::mpsc::{self, Receiver},
        thread,
    },
};

use crate::{env, utils::new_rpc_client};

pub fn monitor_blocktime() -> Receiver<i64> {
    let (blocktime_sender, blocktime_receiver) = mpsc::channel::<i64>();
    thread::spawn(move || {
        let mut latest_blocktime: i64 = 0;

        // Rpc client
        let client = new_rpc_client();

        // Websocket client
        let (_ws_client, slot_receiver) =
            PubsubClient::slot_subscribe(env::wss_endpoint().as_str().into()).unwrap();

        // Listen for new slots
        for slot_info in slot_receiver {
            let blocktime = client.get_block_time(slot_info.slot).unwrap();

            // Publish updated blocktimes
            if blocktime > latest_blocktime {
                latest_blocktime = blocktime;
                blocktime_sender.send(blocktime).unwrap();
            }
        }
    });
    return blocktime_receiver;
}
