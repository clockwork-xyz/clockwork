use std::str::FromStr;

use anchor_lang::solana_program::pubkey::Pubkey;
use clockwork_thread_program_v2::state::VersionedThread;
use dioxus::prelude::*;
use dioxus_router::use_route;
use solana_client_wasm::solana_sdk::account::Account;

use crate::{
    clockwork::get_thread,
    components::{thread_info_table::ThreadInfoTable, thread_sim_logs::ThreadSimLogs},
};

use super::Page;

pub fn ThreadPage(cx: Scope) -> Element {
    let route = use_route(cx);
    let thread = use_state::<Option<(VersionedThread, Account)>>(cx, || None);

    use_future(&cx, (), |_| {
        let thread = thread.clone();
        let thread_pubkey = Pubkey::from_str(route.last_segment().unwrap()).unwrap();
        async move {
            let t = get_thread(thread_pubkey).await;
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
