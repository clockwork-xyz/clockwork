use {
    chrono::{TimeZone, Utc},
    cronos_sdk::account::Health,
    dotenv::dotenv,
    elasticsearch::{auth::Credentials, http::{transport::Transport}, IndexParts, Elasticsearch, Error},
    serde_json::json,
    solana_client_helpers::{Client, RpcClient},
    solana_sdk::{commitment_config::CommitmentConfig, signature::read_keypair},
    std::{fs::File, result::Result, sync::Arc},
};

mod env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    monitor_health_telemetry().await;
    Ok(())
}

async fn monitor_health_telemetry() {
    let mut latest_ts: i64 = 0;
    let es_client = &elastic_client().unwrap();
    let client = &Arc::new(new_client());
    let time_receiver = cronos_sdk::clock::monitor_time(env::wss_endpoint().as_str().into());

    for ts in time_receiver {
        if ts > latest_ts {
            latest_ts = ts;
            record_health_data(client, es_client, ts).await;
        }
    }
}

async fn record_health_data(client: &Arc<Client>, es_client: &Elasticsearch, ts: i64) {
    let health_pubkey = Health::pda().0;
    let health_account = client.get_account_data(&health_pubkey).unwrap();
    let health_data = Health::try_from(health_account).unwrap();

    let last_ping = ts - health_data.last_ping;
    let recurr_drift = ts - health_data.target_ping;
    let ts = Utc.timestamp(ts, 0).naive_utc();

    println!("       Clock: {}", ts);
    println!("   Last ping: {} sec", last_ping);
    println!("Recurr drift: {} sec", recurr_drift);

    es_client
        .index(IndexParts::IndexId(
            env::es_health_index().as_str(),
            &ts.to_string(),
        ))
        .body(json!({
            "clock": ts,
            "last_ping": last_ping,
            "drift": recurr_drift
        }))
        .send()
        .await
        .unwrap();
}

fn new_client() -> Client {
    let payer = read_keypair(&mut File::open(env::keypath().as_str()).unwrap()).unwrap();
    let client = RpcClient::new_with_commitment::<String>(
        env::rpc_endpoint().as_str().into(),
        CommitmentConfig::processed(),
    );
    Client { client, payer }
}

fn elastic_client() -> Result<Elasticsearch, Error> {
    let cloud_id = env::es_cloud_id();
    let credentials = Credentials::Basic(env::es_user(), env::es_password());
    let transport = Transport::cloud(&cloud_id, credentials)?;
    let client = Elasticsearch::new(transport);
    Ok(client)
}
