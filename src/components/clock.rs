use chrono::Utc;
use dioxus::prelude::*;
use solana_client_wasm::{solana_sdk::commitment_config::CommitmentConfig, WasmClient};

pub fn Clock(cx: Scope) -> Element {
    let slot = use_state(&cx, || 0);
    let time = use_state(&cx, || Utc::now());

    use_future(&cx, (), |_| {
        let slot = slot.clone();
        let time = time.clone();
        let client = WasmClient::new("https://api.devnet.solana.com");
        async move {
            loop {
                time.set(Utc::now());
                slot.set(
                    client
                        .get_slot_with_commitment(CommitmentConfig::processed())
                        .await
                        .unwrap(),
                );
                gloo_timers::future::TimeoutFuture::new(1000).await;
            }
        }
    });

    cx.render(rsx! {
        p {
            class: "fixed bottom-0 right-0 p-4",
            format!("Slot: {} {}", slot, time.to_rfc3339())
        }
    })
}
