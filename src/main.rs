use solana_client::{pubsub_client::PubsubClient, rpc_client::RpcClient};
use solana_client_helpers::{Client, ClientResult};
use solana_sdk::{commitment_config::CommitmentConfig, signature::Keypair};

// Solana cluster montior. Replace this with your RPC node.
const SOLANA_CLUSTER: &str = "api.devnet.solana.com";

fn main() -> ClientResult<()> {
    // Initialize rpc client.
    let payer = Keypair::new();
    let client = RpcClient::new_with_commitment(
        format!("https://{}", SOLANA_CLUSTER).as_str().into(),
        CommitmentConfig::confirmed(),
    );
    let client = Client { client, payer };

    // Initialize websocket client.
    let (_client, receiver) =
        PubsubClient::slot_subscribe(format!("ws://{}", SOLANA_CLUSTER).as_str().into()).unwrap();

    // Airdrop to payer.
    client.airdrop(&client.payer_pubkey(), 10_000_000_000)?;

    // Track the latest blocktime.
    let mut latest_blocktime: i64 = 0;

    // Monitor slot updates.
    loop {
        match receiver.recv() {
            Ok(slot_info) => {
                let blocktime = client.get_block_time(slot_info.slot).unwrap();
                if blocktime > latest_blocktime {
                    latest_blocktime = blocktime;
                }
                println!("Slot: {:?}  Blocktime: {:?}", slot_info.slot, blocktime);
            }
            Err(err) => {
                println!("Err: {:?}", err);
            }
        }
    }
}
