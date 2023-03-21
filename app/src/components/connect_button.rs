use std::str::FromStr;

use clockwork_utils::pubkey::Abbreviated;
use dioxus::prelude::*;
use solana_client_wasm::{solana_sdk::pubkey::Pubkey, WasmClient};

use super::backpack::backpack;
use crate::context::User;

pub fn ConnectButton(cx: Scope) -> Element {
    let cx = cx.clone();
    let user_context = use_shared_state::<User>(cx).unwrap();

    let handle_click = move |_| {
        cx.spawn({
            let client = WasmClient::new("http://74.118.139.244:8899");
            let user_context = user_context.clone();
            async move {
                let user_context_read = user_context.read();
                match user_context_read.account.is_some() {
                    true => {
                        let response = backpack.disconnect().await;
                        log::info!("disconnected: {:?}", response);
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
                                    log::info!("pubkey: {}, account: {:#?}", pubkey, acc);
                                }
                                Err(err) => log::info!("Failed to get user account: {:?}", err),
                            }
                        }
                    }
                }
            }
        });
    };

    let connect_text = if let Some(pubkey) = user_context.read().pubkey {
        pubkey.abbreviated()
    } else {
        String::from("Connect")
    };

    cx.render(rsx! {
        button {
            class: "px-6 py-3 border rounded-full transition text-slate-100 hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 font-semibold",
            onclick: handle_click,
            connect_text
        }
    })
}
