use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use clockwork_relayer_api::{SecretCreate, SignedRequest};
use dioxus::prelude::*;
use dioxus_router::use_router;
use reqwest::header::CONTENT_TYPE;
use solana_client_wasm::solana_sdk::signature::Signature;

use crate::components::backpack;

use super::Page;

pub fn NewKeyPage(cx: Scope) -> Element {
    let router = use_router(&cx);

    let name = use_state(cx, || "".to_string());
    let word = use_state(cx, || "".to_string());

    // TODO: Connect a new key to the user profile.

    cx.render(rsx! {
        Page {
            div {
                class: "flex justify-around h-full w-full",
                div {
                    class: "flex flex-col m-auto w-full max-w-3xl space-y-6",
                    h1 {
                        class: "text-2xl font-semibold",
                        "New Key"
                    }
                    form {
                        class: "flex flex-col space-y-8 w-full",
                        div {
                            p {
                                class: "text-left text-sm font-medium mb-1",
                                "Name"
                            }
                            input {
                                class: "bg-transparent border-b text-base font-normal py-3 px-3 w-full hover:bg-slate-100 hover:text-slate-900",
                                r#type: "text",
                                name: "Name",
                                value: "{name}",
                                oninput: move |e| name.set(e.value.clone()),
                            }
                        }
                        div {
                            p {
                                class: "text-left text-sm font-medium mb-1",
                                "Value"
                            }
                            input {
                                class: "bg-transparent border-b text-base font-normal py-3 px-3 w-full hover:bg-slate-100 hover:text-slate-900",
                                r#type: "text",
                                name: "Value",
                                value: "{word}",
                                oninput: move |e| word.set(e.value.clone()),
                            }
                        }
                    }
                    div {
                        class: "flex flex-row w-full justify-between",
                        button {
                            class: "font-normal text-slate-100 bg-transparent hover:bg-slate-100 hover:text-slate-900 transition py-3 w-full",
                            onclick: move |_| { router.navigate_to("/keys") },
                            "Cancel"
                        }
                        button {
                            class: "font-semibold text-slate-100 bg-transparent hover:bg-slate-100 hover:text-slate-900 transition py-3 w-full",
                            onclick: |_| {
                                let name = name.clone();
                                let word = word.clone();
                                async move {
                                    create_secret(name.get().clone(), word.get().clone()).await;
                                }
                            },
                            "Continue"
                        }
                    }
                }
            }
        }
    })
}

pub async fn create_secret(name: String, word: String) -> String {
    let msg = SecretCreate { name, word };
    let msg_bytes = bincode::serialize(&msg).unwrap();
    let pubkey = Pubkey::from_str(backpack.pubkey().to_string().as_str()).unwrap();
    let req = SignedRequest {
        msg,
        signer: pubkey,
        signature: Signature::new(
            &*js_sys::Uint8Array::new(
                &(backpack
                    .sign_message(msg_bytes, Some(backpack.pubkey()))
                    .await),
            )
            .to_vec(),
        ),
    };
    reqwest::Client::new()
        .post("http://3.83.67.25:8000/secret_create")
        .header(CONTENT_TYPE, "application/json")
        .json(&req)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap()
}
