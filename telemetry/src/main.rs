use {
    crate::{client::RPCClient, env::Envvar},
    bincode::deserialize,
    chrono::{TimeZone, Utc},
    cronos_sdk::healthcheck::state::Health,
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
            record_health_data().await;
        }
    });
    let _ = forever.await;
    Ok(())
}

async fn record_health_data() {
    // Build clients
    let client = Client::new(Envvar::Keypath.get(), Envvar::RpcEndpoint.get());
    let es_client = elastic_client().unwrap();

    // Get clock data
    let clock_pubkey = Pubkey::from_str("SysvarC1ock11111111111111111111111111111111").unwrap();
    let clock_data = client.get_account_data(&clock_pubkey).unwrap();
    let clock_data = deserialize::<Clock>(&clock_data).unwrap();
    let ts = clock_data.unix_timestamp;

    // Get health data
    let health_data = Health::try_from(client.get_account_data(&Health::pda().0).unwrap()).unwrap();
    let last_ping = ts - health_data.last_ping;
    let ts = Utc.timestamp(ts, 0).naive_utc();

    // Pipe data to elasticsearch
    es_client
        .index(IndexParts::IndexId(
            Envvar::EsIndex.get().as_str(),
            &ts.to_string(),
        ))
        .body(json!({
            "clock": ts,
            "last_ping": last_ping,
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
