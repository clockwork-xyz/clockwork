use {
    crate::{cache::TaskCache, env},
    cronos_sdk::cronos::state::*,
    solana_account_decoder::UiAccountEncoding,
    solana_client::{
        pubsub_client::PubsubClient,
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
    },
    solana_sdk::{account::Account, commitment_config::CommitmentConfig, pubkey::Pubkey},
    std::{
        str::FromStr,
        sync::{Arc, RwLock},
        thread,
    },
};

pub fn replicate_tasks(cache: Arc<RwLock<TaskCache>>) {
    thread::spawn(move || {
        // Websocket client
        let (_ws_client, keyed_account_receiver) = PubsubClient::program_subscribe(
            env::wss_endpoint().as_str().into(),
            &cronos_sdk::ID,
            Some(RpcProgramAccountsConfig {
                account_config: RpcAccountInfoConfig {
                    encoding: Some(UiAccountEncoding::Base64),
                    commitment: Some(CommitmentConfig::processed()),
                    data_slice: None,
                },
                filters: None,
                with_context: None,
            }),
        )
        .unwrap();

        // Listen for new accounts
        for keyed_account_response in keyed_account_receiver {
            let keyed_account = keyed_account_response.value;
            let account = keyed_account.account.decode::<Account>().unwrap();

            // Unwrap task
            let task = Task::try_from(account.data);
            if !task.is_err() {
                let key = Pubkey::from_str(&keyed_account.pubkey).unwrap();
                let task = task.unwrap();
                println!("💽 Replicating task {}", key);
                let mut w_cache = cache.write().unwrap();
                if account.lamports == 0 {
                    w_cache.delete(key)
                } else {
                    w_cache.insert(key, task)
                }
            }
        }

        // If we reach here, just restart the process.
        replicate_tasks(cache);
    });
}
