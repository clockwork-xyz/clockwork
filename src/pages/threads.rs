use anchor_lang::Discriminator;
use clockwork_sdk::state::Thread;
use dioxus::prelude::*;
use log::info;
use solana_client_wasm::{
    utils::{
        rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
        rpc_filter::{Memcmp, MemcmpEncodedBytes, RpcFilterType},
    },
    WasmClient,
};

use super::Page;

pub fn ThreadsPage(cx: Scope) -> Element {
    let _threads = use_state::<Vec<Thread>>(&cx, || vec![]);

    use_future(&cx, (), |_| {
        let client = WasmClient::new("http://74.118.139.244:8899");
        async move {
            let accounts = client
                .get_program_accounts_with_config(
                    &clockwork_sdk::ID,
                    RpcProgramAccountsConfig {
                        filters: Some(vec![RpcFilterType::Memcmp(Memcmp {
                            offset: 0,
                            bytes: MemcmpEncodedBytes::Bytes(Thread::discriminator().to_vec()),
                            encoding: None,
                        })]),
                        account_config: RpcAccountInfoConfig::default(),
                        with_context: None,
                    },
                )
                .await
                .unwrap();
            info!("{:?}", accounts);
        }
    });

    cx.render(rsx! {
        Page {
            h1 {
                class: "text-2xl font-semibold",
                "Threads"
            }
        }
    })
}
