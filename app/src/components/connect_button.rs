use std::str::FromStr;

use clockwork_utils::pubkey::Abbreviated;
use dioxus::prelude::*;
use gloo_events::EventListener;
use gloo_storage::{LocalStorage, Storage};
use solana_client_wasm::{
    solana_sdk::pubkey::Pubkey,
    WasmClient,
};

use super::backpack::backpack;
use crate::{
    context::{Cluster, User},
    utils::format_balance,
};

pub fn ConnectButton(cx: Scope) -> Element {
    let user_context = use_shared_state::<User>(cx).unwrap();
    let show_popover = use_state(&cx, || false);
    let show_cluster_dropdown = use_state(&cx, || false);

    {
        let user_context = user_context.clone();
        use_effect(cx, (), |_| async move {
            match LocalStorage::get::<User>("user_context") {
                Ok(u) => {
                    let mut uc_write = user_context.write();
                    uc_write.account = u.account;
                    uc_write.pubkey = u.pubkey;
                    uc_write.cluster = u.cluster;
                }
                Err(_err) => log::info!("user is not logged in"),
            }
        });
    }

    let handle_click = move |_| {
        cx.spawn({
            let client = WasmClient::new("http://74.118.139.244:8899");
            let user_context = user_context.clone();
            let show_popover = show_popover.clone();

            async move {
                let user_context_read = user_context.read();
                match user_context_read.account.is_some() {
                    true => {
                        show_popover.set(!*show_popover.get());
                    }
                    _ => {
                        backpack.connect().await;
                        log::info!("connected: {:?}", backpack.is_connected());
                        if backpack.is_connected() {
                            let pubkey =
                                Pubkey::from_str(backpack.pubkey().to_string().as_str()).unwrap();
                            let account = client.get_account(&pubkey).await;
                            match account {
                                Ok(acc) => {
                                    drop(user_context_read);
                                    user_context.write().account = Some(acc.clone());
                                    user_context.write().pubkey = Some(pubkey);
                                    LocalStorage::set(
                                        "user_context",
                                        User {
                                            pubkey: user_context.read().pubkey,
                                            account: user_context.read().account.clone(),
                                            cluster: user_context.read().cluster.clone(),
                                        },
                                    )
                                    .unwrap();
                                }
                                Err(err) => log::info!("Failed to get user account: {:?}", err),
                            }
                        }
                    }
                }
            }
        });
    };

    use_future(&cx, (), |_| {
        let user_context = user_context.clone();
        let show_cluster_dropdown = show_cluster_dropdown.clone();
        async move {
            let document = gloo_utils::document();
            Some(EventListener::new(&document, "click", move |_| {
                let document = gloo_utils::document();
                if let Some(element) = document.active_element() {
                    let element_id = element.id();
                    let e_id = element_id.as_str();
                    match e_id {
                        "Mainnet" | "Devnet" | "Testnet" => {
                            user_context.write().cluster = Cluster::from_str(e_id).unwrap();
                            LocalStorage::set(
                                "user_context",
                                User {
                                    pubkey: user_context.read().pubkey,
                                    account: user_context.read().account.clone(),
                                    cluster: user_context.read().cluster.clone(),
                                },
                            )
                            .unwrap();
                            show_cluster_dropdown.set(false);
                        }
                        _ => {}
                    }
                }
            }))
        }
    });

    let connect_text = if let Some(pubkey) = user_context.read().pubkey {
        pubkey.abbreviated()
    } else {
        String::from("Connect")
    };

    cx.render(rsx! {
        button {
            class: "px-6 py-3 border rounded-full transition text-slate-100 hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 font-semibold",
            onclick: handle_click,
            connect_text.as_str()
        }
        if *show_popover.get() {
            rsx! {
                div {
                    class: "absolute top-[60px] right-[34px] transform overflow-hidden h-60 flex flex-col rounded-lg bg-slate-800 px-4 pt-5 items-center pb-4 space-y-4 shadow-xl transition-all sm:my-8 sm:w-full sm:max-w-sm sm:p-6",
                    div {
                        class: "w-full flex flex-row justify-between",
                        p {
                            class: "text-slate-100 font-semibold",
                            "{connect_text.as_str()}"
                        }
                        Balance {}
                    }
                    button {
                        class: "inline-flex w-full justify-center gap-x-1.5 rounded-md bg-white px-3 py-2 text-sm font-semibold text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50",
                        onclick: move |_| { show_cluster_dropdown.set(true) },
                        "{user_context.read().cluster.to_string()}"
                        svg {
                            class: "-mr-1 h-5 w-5 text-slate-800", 
                            view_box: "0 0 20 20",
                            fill: "currentColor",
                            path {
                                fill_rule: "evenodd", 
                                d: "M5.23 7.21a.75.75 0 011.06.02L10 11.168l3.71-3.938a.75.75 0 111.08 1.04l-4.25 4.5a.75.75 0 01-1.08 0l-4.25-4.5a.75.75 0 01.02-1.06z", 
                            }
                        }
                    }
                    if *show_cluster_dropdown.get() {
                        rsx! {
                            div {
                                id: "div1",
                                class: "absolute top-[90px] right-[25px] z-0 mt-2 w-[335px] origin-top-right rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none",
                                div {
                                    id: "div2",
                                    class: "py-1",
                                    button {
                                        class: "text-slate-800 block px-4 py-2 text-sm",
                                        id: "Mainnet",
                                        "Mainnet"
                                    }
                                    button {
                                        class: "text-slate-800 block px-4 py-2 text-sm",
                                        id: "Devnet",
                                        "Devnet"
                                    }
                                    button {
                                        class: "text-slate-800 block px-4 py-2 text-sm",
                                        id: "Testnet",
                                        "Testnet",
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

fn Balance(cx: Scope) -> Element {
    let user_context = use_shared_state::<User>(cx).unwrap();

    let user_balance = if let Some(account) = &user_context.read().account {
        format_balance(account.lamports, true)
    } else {
        String::from("")
    };

    cx.render(rsx! {
        div {
            class: "text-lg",
            user_balance
        }
    })
}
