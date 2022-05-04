use {
    crate::{client::RPCClient, env::Envvar},
    bincode::deserialize,
    chrono::{TimeZone, Utc},
    cronos_sdk::heartbeat::state::Heartbeat,
    dotenv::dotenv,
    elasticsearch::{
        auth::Credentials, http::transport::Transport, Elasticsearch, Error, IndexParts,
    },
    serde_json::json,
    solana_client_helpers::Client,
    solana_program::clock::Clock,
    solana_sdk::pubkey::Pubkey,
    std::{result::Result, str::FromStr, time::Duration},
    tokio::{task, time},
};

mod client;
mod env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let forever = task::spawn(async {
        let mut interval = time::interval(Duration::from_millis(10_000));
        loop {
            interval.tick().await;
            record_heartbeat().await;
        }
    });
    _ = forever.await;
    Ok(())
}

async fn record_heartbeat() {
    // Build clients
    let client = Client::new(Envvar::Keypath.get(), Envvar::RpcEndpoint.get());
    let es_client = elastic_client().unwrap();

    // Get clock data
    let clock_pubkey = Pubkey::from_str("SysvarC1ock11111111111111111111111111111111").unwrap();
    let clock_data = client.get_account_data(&clock_pubkey).unwrap();
    let clock_data = deserialize::<Clock>(&clock_data).unwrap();
    let ts = clock_data.unix_timestamp;

    // Get heartbeat data
    let heartbeat_pubkey = Heartbeat::pda().0;
    let heartbeat_data =
        Heartbeat::try_from(client.get_account_data(&heartbeat_pubkey).unwrap()).unwrap();

    // Compute telemetry data
    let last_ping = ts - heartbeat_data.last_ping;
    let drift = ts - heartbeat_data.target_ping;
    let ts = Utc.timestamp(ts, 0).naive_utc();

    // Pipe telemetry data to elasticsearch
    es_client
        .index(IndexParts::IndexId(
            Envvar::EsIndex.get().as_str(),
            &ts.to_string(),
        ))
        .body(json!({
            "drift": drift,
            "last_ping": last_ping,
            "ts": ts,
        }))
        .send()
        .await
        .unwrap();
}

fn elastic_client() -> Result<Elasticsearch, Error> {
    let credentials = Credentials::Basic(Envvar::EsUser.get(), Envvar::EsPassword.get());
    let transport = Transport::cloud(&Envvar::EsCloudId.get(), credentials)?;
    let client = Elasticsearch::new(transport);
    Ok(client)
}
