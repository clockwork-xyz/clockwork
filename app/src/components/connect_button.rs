use clockwork_utils::pubkey::Abbreviated;
use dioxus::prelude::*;
use dioxus_router::{use_route, use_router, RouterService};
use gloo_events::EventListener;
use gloo_storage::{LocalStorage, Storage};
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use std::{rc::Rc, str::FromStr};
use url::Url;

use super::backpack::backpack;
use crate::{
    context::{Client, Cluster, User},
    utils::format_balance,
};

pub fn ConnectButton(cx: Scope) -> Element {
    let route = use_route(&cx);
    let router = use_router(&cx);
    let user_context = use_shared_state::<User>(cx).unwrap();
    let client_context = use_shared_state::<Client>(cx).unwrap();
    let show_popover = use_state(&cx, || false);
    let show_cluster_dropdown = use_state(&cx, || false);

    let cluster_query_param = route
        .query_param("cluster")
        .unwrap_or(std::borrow::Cow::Borrowed(""))
        .into_owned();
    let url = Url::parse(&route.url().to_string()).unwrap();
    let current_route = if url.path() == "/" {
        String::from("/accounts")
    } else {
        String::from_str(url.path()).unwrap()
    };

    // handle cluster config with query param and local storage
    use_effect(cx, (&url,), |_| {
        let user_context = user_context.clone();
        let client_context = client_context.clone();
        let current_route = current_route.clone();

        to_owned![router];
        async move {
            if let Ok(user) = LocalStorage::get::<User>("user_context") {
                log::info!("user is logged in!");
                // TODO: refetch user account data/ask user to log in after a certain timeframe
                let mut uc_write = user_context.write();
                uc_write.account = user.account;
                uc_write.pubkey = user.pubkey;
            }
            update_cluster_and_navigate(
                cluster_query_param,
                client_context,
                &router,
                &current_route,
            );
        }
    });

    // wallet connect flow
    let handle_click = move |_| {
        cx.spawn({
            let user_context = user_context.clone();
            let client_context = client_context.clone();
            let show_popover = show_popover.clone();
            let client_context = client_context.clone();
            async move {
                let user_context_read = user_context.read();
                let client_context = client_context.read();
                match user_context_read.account.is_some() {
                    true => {
                        show_popover.set(!*show_popover.get());
                    }
                    _ => {
                        // Check if the provider is not connected before connecting
                        if !backpack.is_connected() {
                            backpack.connect().await;
                        }
                        log::info!("connected: {:?}", backpack.is_connected());
                        if backpack.is_connected() {
                            let pubkey =
                                Pubkey::from_str(backpack.pubkey().to_string().as_str()).unwrap();
                            let account = client_context.client.get_account(&pubkey).await;
                            match account {
                                Ok(acc) => {
                                    drop(user_context_read);
                                    user_context.write().account = Some(acc.clone());
                                    user_context.write().pubkey = Some(pubkey);
                                    log::info!("A");
                                    LocalStorage::set(
                                        "user_context",
                                        User {
                                            pubkey: user_context.read().pubkey,
                                            account: user_context.read().account.clone(),
                                        },
                                    )
                                    .unwrap();
                                    LocalStorage::set(
                                        "cluster",
                                        client_context.cluster.to_string().to_lowercase(),
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
        let client_context = client_context.clone();
        let show_cluster_dropdown = show_cluster_dropdown.clone();
        to_owned![router];

        async move {
            let document = gloo_utils::document();
            Some(EventListener::new(&document, "click", move |_| {
                let document = gloo_utils::document();
                if let Some(element) = document.active_element() {
                    let element_id = element.id();
                    let e_id = element_id.as_str();
                    match e_id {
                        "mainnet" | "devnet" => {
                            let cluster = Cluster::from_str(e_id).unwrap();
                            *client_context.write() = Client::new_with_config(cluster);
                            LocalStorage::set("cluster", cluster.to_string().to_lowercase())
                                .unwrap();
                            router.navigate_to(
                                format!("{}?cluster={}", current_route, cluster.to_string())
                                    .as_str(),
                            );
                            let _ = web_sys::window().unwrap().location().reload();
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

    let current_cluster_text: String = client_context
        .read()
        .cluster
        .to_string()
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i == 0 {
                c.to_uppercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect();
    // log::info!(
    //     "client_context.read().cluster: {}",
    //     client_context.read().cluster.to_string()
    // );

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
                        "{current_cluster_text}"
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
                                        id: "mainnet",
                                        "Mainnet"
                                    }
                                    button {
                                        class: "text-slate-800 block px-4 py-2 text-sm",
                                        id: "devnet",
                                        "Devnet"
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

fn update_cluster_and_navigate(
    cluster_query_param: String,
    client_context: UseSharedState<Client>,
    router: &Rc<RouterService>,
    current_route: &str,
) {
    let cluster_from_query_param = get_cluster_from_query_param(cluster_query_param);
    let cached_cluster = get_cached_cluster();

    let cluster_to_use = cluster_from_query_param
        .or(cached_cluster
            .as_ref()
            .map(|s| Cluster::from_str(&s))
            .transpose()
            .unwrap())
        .unwrap_or_else(|| Cluster::from_str("mainnet").unwrap());

    update_client_context_and_cache(client_context, cluster_to_use);
    navigate_to_route_with_cluster(router, current_route, &cluster_to_use.to_string());
}

fn get_cluster_from_query_param(query_param: String) -> Option<Cluster> {
    if query_param.eq("devnet") || query_param.eq("mainnet") {
        Cluster::from_str(&query_param).ok()
    } else {
        None
    }
}

fn get_cached_cluster() -> Option<String> {
    LocalStorage::get::<String>("cluster").ok()
}

fn update_client_context_and_cache(client_context: UseSharedState<Client>, cluster: Cluster) {
    log::info!("updating cluster cache to: {}", cluster.to_string());
    *client_context.write() = Client::new_with_config(cluster);
    LocalStorage::set("cluster", &cluster.to_string().to_lowercase()).unwrap();
}

fn navigate_to_route_with_cluster(
    router: &Rc<RouterService>,
    current_route: &str,
    cluster_query_param: &str,
) {
    router.navigate_to(format!("{}?cluster={}", current_route, cluster_query_param).as_str());
}
