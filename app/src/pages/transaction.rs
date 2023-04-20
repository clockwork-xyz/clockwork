use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_router::use_route;
use solana_client_wasm::solana_sdk::signature::Signature;
use solana_extra_wasm::transaction_status::EncodedConfirmedTransactionWithStatusMeta;

use crate::{components::TransactionInfo, context::Client, pages::page::Page};

pub fn TransactionPage(cx: Scope) -> Element {
    let route = use_route(cx);
    let transaction = use_state::<Option<EncodedConfirmedTransactionWithStatusMeta>>(cx, || None);
    let client_context = use_shared_state::<Client>(cx).unwrap();
    use_future(&cx, (), |_| {
        let transaction = transaction.clone();
        let client_context = client_context.clone();
        let transaction_signature = Signature::from_str(route.last_segment().unwrap()).unwrap();
        async move {
            let t = client_context
                .read()
                .get_account_transaction(&transaction_signature)
                .await
                .unwrap();
            transaction.set(Some(t.clone()));
        }
    });

    if let Some(t) = transaction.get() {
        cx.render(rsx! {
            Page {
                div {
                    class: "flex flex-col space-y-16",
                    div {
                        class: "flex flex-col justify-between",
                        h1 {
                             class: "text-2xl font-semibold mb-6",
                             "TRANSACTION"
                        }
                        TransactionInfo { data: t.clone() }
                    }
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
