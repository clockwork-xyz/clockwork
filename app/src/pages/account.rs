use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use dioxus::prelude::*;
use dioxus_router::use_route;
use solana_client_wasm::solana_sdk::account::Account;

use crate::{clockwork::get_account, components::account_info_table::AccountInfoTable};

use super::Page;

pub fn AccountPage(cx: Scope) -> Element {
    let route = use_route(cx);
    let account = use_state::<Option<Account>>(cx, || None);

    // TODO Unwrap address safely
    let address = Pubkey::from_str(route.last_segment().unwrap()).unwrap();

    use_future(&cx, (), |_| {
        let account = account.clone();
        async move {
            log::info!("Address: {:?}", address);
            match get_account(address).await {
                Ok(maybe_account) => {
                    account.set(maybe_account);
                }
                Err(err) => {
                    // TODO Handle error
                }
            }
        }
    });

    log::info!("Account: {:?}", account.get());
    cx.render(rsx! {
        Page {
            div {
                class: "flex flex-col",
                h1 {
                     class: "text-2xl font-semibold mb-6",
                     "Account"
                }
                if let Some(account) = account.get() {
                    rsx! {
                        AccountInfoTable {
                            account: account.clone(),
                            address: address,
                        }
                    }
                } else {
                    rsx! {
                        "Loading..."
                    }
                }
            }
        }
    })
}
