use std::str::FromStr;

use anchor_lang::solana_program::pubkey::Pubkey;
use clockwork_thread_program_v2::state::VersionedThread;
use dioxus::prelude::*;
use dioxus_router::use_route;
use solana_client_wasm::{
    solana_sdk::account::Account,
};

use crate::{
    components::{
        thread_info_table::ThreadInfoTable, thread_sim_logs::ThreadSimLogs, TransactionHistoryTable,
    },
    context::Client,
};

use super::Page;
use crate::context::Cluster;

pub fn ThreadPage(cx: Scope) -> Element {
    let route = use_route(cx);
    let thread = use_state::<Option<(VersionedThread, Account)>>(cx, || None);
    let client_context = use_shared_state::<Client>(cx).unwrap();

    use_future(&cx, (), |_| {
        let thread = thread.clone();
        let client_context = client_context.clone();
        let thread_pubkey = Pubkey::from_str(route.last_segment().unwrap()).unwrap();
        async move {
            let cluster_config = client_context.read().cluster.clone();
            drop(client_context);
            let t = get_thread(thread_pubkey, cluster_config).await;
            thread.set(Some(t.clone()));
        }
    });

    if let Some(t) = thread.get() {
        cx.render(rsx! {
            Page {
                div {
                    class: "flex flex-col space-y-16",
                    div {
                        class: "flex flex-col justify-between",
                        h1 {
                             class: "text-2xl font-semibold mb-6",
                             "Thread"
                        }
                        ThreadInfoTable { account: t.clone().1, thread: t.clone().0 }
                    }
                    ThreadSimLogs { thread: t.clone().0 }
                    TransactionHistoryTable { address: t.clone().0.pubkey() }
                }
            }
        })
    } else {
        cx.render(rsx! {
            Page {
                div {
                    "Loading..."
                }
            }
        })
    }
}

async fn get_thread(thread_pubkey: Pubkey, cluster: Cluster) -> (VersionedThread, Account) {
    let client = Client::new_with_config(cluster);
    client.get_thread(thread_pubkey).await.unwrap()
}
