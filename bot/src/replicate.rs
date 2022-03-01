// use anchor_lang::prelude::Pubkey;

use cronos_sdk::account::*;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    pubsub_client::PubsubClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
};
use solana_sdk::{account::Account, commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::{str::FromStr, thread, sync::{Arc, RwLock}};

use crate::{env, store::{TaskStore, MutableTaskStore}};

pub fn replicate_cronos_tasks(store: Arc<RwLock<TaskStore>>) {
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
                let task = task.unwrap();
                let mut w_store = store.write().unwrap();
                w_store.insert(Pubkey::from_str(&keyed_account.pubkey).unwrap(), task)
            }
        }

        // If we reach here, just restart the process.
        replicate_cronos_tasks(store);
    });
}
