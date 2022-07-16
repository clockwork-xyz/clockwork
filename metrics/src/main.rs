use {
    crate::env::Envvar,
    chrono::{TimeZone, Utc},
    cronos_client::health::state::Health,
    cronos_client::Client,
    dotenv::dotenv,
    elasticsearch::{
        auth::Credentials, http::transport::Transport, Elasticsearch, Error, IndexParts,
    },
    serde_json::json,
    solana_sdk::signature::read_keypair_file,
    std::{result::Result, time::Duration},
    tokio::{task, time},
};

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
    let payer = read_keypair_file(Envvar::Keypath.get()).unwrap();
    let client = Client::new(payer, Envvar::RpcEndpoint.get());
    let es_client = elastic_client().unwrap();

    // Get data
    let clock = client.get_clock().unwrap();
    let health = client.get::<Health>(&Health::pubkey()).unwrap();

    // Pipe data to elasticsearch
    let last_ping = clock.unix_timestamp - health.last_ping;
    let ts = Utc.timestamp(clock.unix_timestamp, 0).naive_utc();
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
