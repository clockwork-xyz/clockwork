use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use clockwork_relayer_api::{SecretList, SecretListResponse, SignedRequest};
use dioxus::prelude::*;
use dioxus_router::Link;
use dotenv_codegen::dotenv;
use reqwest::header::CONTENT_TYPE;
use solana_client_wasm::solana_sdk::signature::Signature;

use super::Page;

pub fn SecretsPage(cx: Scope) -> Element {
    let secrets = use_state::<Vec<String>>(&cx, || vec![]);

    use_future(&cx, (), |_| {
        let secrets = secrets.clone();
        async move {
            if backpack.is_connected() {
                secrets.set(get_secrets().await)
            }
        }
    });

    cx.render(rsx! {
        Page {
            div {
                class: "flex flex-row justify-between mb-6",
                h1 {
                    class: "text-2xl font-semibold",
                    "Secrets"
                }
                Link {
                    to: "/secrets/new"
                    class: "hover:bg-white hover:text-slate-900 text-slate-100 font-semibold py-3 px-6 hover:bg-slate-200 transition",
                    "New secret"
                }
            }
            if secrets.is_empty() {
                rsx! {
                    h1 {
                        class: "text-slate-600 text-base",
                        "No secrets"
                    }
                }
            }
            for secret in secrets.get() {
                h1 {
                    "{secret}"
                }
            }
        }
    })
}

use crate::backpack::backpack;

pub async fn get_secrets() -> Vec<String> {
    let msg = SecretList {};
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
    match reqwest::Client::new()
        .post(format!("{}/secret_list", dotenv!("RELAYER_URL")))
        .header(CONTENT_TYPE, "application/json")
        .json(&req)
        .send()
        .await
    {
        Ok(res) => res.json::<SecretListResponse>().await.unwrap().secrets,
        Err(_err) => vec![],
    }
}
