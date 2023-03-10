use dioxus::prelude::*;
use dotenv_codegen::dotenv;
use solana_client_wasm::{
    solana_sdk::commitment_config::CommitmentConfig, utils::rpc_config::RpcBlockConfig, WasmClient,
};
use solana_extra_wasm::transaction_status::UiConfirmedBlock;

pub fn BlocksTable(cx: Scope) -> Element {
    let block = use_state::<Option<UiConfirmedBlock>>(&cx, || None);

    use_future(&cx, (), |_| {
        let block = block.clone();
        async move {
            loop {
                if let Some(recent_block) = get_block().await {
                    block.set(Some(recent_block));
                }
                gloo_timers::future::TimeoutFuture::new(1000).await;
            }
        }
    });

    cx.render(rsx! {
        div {
            h1 {
                class: "text-2xl font-semibold mb-6",
                "Blocks"
            }
            table {
                class: "w-full divide-y divide-slate-800",
                Header {}
                if let Some(block) = block.get() {
                    rsx! {
                        tr {
                            class: "px-3 text-base hover:bg-slate-100 hover:text-slate-900 hover:cursor-pointer focus:bg-slate-900",
                            td {
                                class: "whitespace-nowrap px-3 py-4",
                                "{block.blockhash}"
                            }
                            td {
                                class: "whitespace-nowrap px-3 py-4",
                                "{block.signatures.as_ref().unwrap_or(&vec![]).len()}"
                            }
                        }
                    }
                }
            }
        }
    })
}

fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        thead {
            tr {
                class: "text-left text-sm text-slate-500",
                th {
                    class: "py-3 px-3 font-medium",
                    scope: "col",
                    "Blockhash"
                }
                th {
                    class: "py-3 px-3 font-medium",
                    scope: "col",
                    "Transactions"
                }
            }
        }
    })
}

pub async fn get_block() -> Option<UiConfirmedBlock> {
    const HELIUS_API_KEY: &str = dotenv!("HELIUS_API_KEY");
    let url = format!("https://rpc.helius.xyz/?api-key={}", HELIUS_API_KEY);
    let helius_rpc_endpoint = url.as_str();
    let client = WasmClient::new(helius_rpc_endpoint);
    let slot = client
        .get_latest_blockhash_with_commitment(CommitmentConfig::processed())
        .await
        .unwrap()
        .1;
    client
        .get_block_with_config(
            slot,
            RpcBlockConfig {
                encoding: None,
                transaction_details: Some(
                    solana_extra_wasm::transaction_status::TransactionDetails::Signatures,
                ),
                rewards: Some(true),
                commitment: Some(CommitmentConfig::processed()),
                max_supported_transaction_version: None,
            },
        )
        .await
        .ok()
}
