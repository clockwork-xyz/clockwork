use chrono::Utc;
use dioxus::prelude::*;
use solana_client_wasm::WasmClient;

pub fn Clock(cx: Scope) -> Element {
    let blockhash = use_state(&cx, || String::new());
    let slot = use_state(&cx, || 0);
    let time = use_state(&cx, || Utc::now());

    use_future(&cx, (), |_| {
        let blockhash = blockhash.clone();
        let slot = slot.clone();
        let time = time.clone();
        let client = WasmClient::new("http://74.118.139.244:8899");
        async move {
            loop {
                blockhash.set(client.get_latest_blockhash().await.unwrap().to_string());
                time.set(Utc::now());
                slot.set(client.get_slot().await.unwrap());
                gloo_timers::future::TimeoutFuture::new(1000).await;
            }
        }
    });

    cx.render(rsx! {
        p {
            class: "fixed bottom-0 right-0 p-4",
            format!("Blockhash: {} Slot: {} {}", blockhash, slot, time.to_rfc3339())
        }
    })
}
