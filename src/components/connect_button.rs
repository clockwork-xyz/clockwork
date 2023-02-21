use std::str::FromStr;

use dioxus::prelude::*;
use log::{self};
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
use std::ops::Deref;
use wasm_bindgen_futures::spawn_local;

use super::backpack::backpack;

pub fn ConnectButton(cx: Scope) -> Element {
    let account_handle = use_state(&cx, || None);
    let account = account_handle.deref().clone();
    // let has_account = account.trim().is_empty();

    let handle_click = |_| {
        let account_handle = account_handle.clone();
        let account = account_handle.deref().clone();

        spawn_local(async move {
            match account.is_some() {
                true => {
                    let response = backpack.disconnect().await;
                    log::info!("disconnected: {:?}", response);
                    account_handle.set(None);
                }
                _ => {
                    backpack.connect().await;
                    log::info!("connected: {:?}", backpack.is_connected());
                    if backpack.is_connected() {
                        let pubkey =
                            Pubkey::from_str(backpack.pubkey().to_string().as_str()).unwrap();
                        account_handle.set(Some(pubkey));
                    }
                }
            };
        });
    };

    let connect_text = if let Some(pubkey) = account {
        pubkey.abbreviated()
    } else {
        String::from("Connect")
    };

    cx.render(rsx! {
        button {
            class: "px-4 py-2 border rounded-full",
            onclick: handle_click,
            connect_text
        }
    })
}

trait Abbreviated {
    fn abbreviated(&self) -> String;
}

impl Abbreviated for Pubkey {
    fn abbreviated(&self) -> String {
        let s = self.to_string();
        let len = s.len();
        format!(
            "{}...{}",
            s.get(0..4).unwrap(),
            s.get(len - 4..len).unwrap()
        )
        .to_string()
    }
}
