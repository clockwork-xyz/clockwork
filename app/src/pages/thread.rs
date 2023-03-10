use std::str::FromStr;

use std::convert::From;
use super::Page;
use crate::clockwork::{get_thread, simulate_thread};
use anchor_lang::solana_program::{pubkey::Pubkey, instruction::Instruction};
use clockwork_sdk::state::Thread;
use dioxus::prelude::*;
use dioxus_router::use_route;
use dotenv_codegen::dotenv;
use solana_client_wasm::{WasmClient, solana_sdk::{compute_budget::{self, ComputeBudgetInstruction}, transaction::TransactionError}};

pub fn ThreadPage(cx: Scope) -> Element {
    let route = use_route(cx);
    let thread = use_state::<Option<Thread>>(cx, || None);
    let thread_pubkey = Pubkey::from_str(route.last_segment().unwrap()).unwrap();
    let show_modal = use_state::<bool>(cx, || false);
    let logs = use_state::<Vec<String>>(cx, || vec![]);
    let log_errors = use_state::<Option<TransactionError>>(cx, || None);

    use_future(&cx, (), |_| {
        let thread = thread.clone();
        let thread_pubkey = thread_pubkey.clone();
        async move { thread.set(Some(get_thread(thread_pubkey).await)) }
    });

    let handle_click = move |_| {
        cx.spawn({

            let thread = thread.clone();
            let show_modal = show_modal.clone();
            let logs = logs.clone();
            let log_errors = log_errors.clone();
            show_modal.set(true);
            async move {
                if let Some(t) = thread.get() {
                    match simulate_thread(t.to_owned()).await {
                        Ok(res) => {
                            logs.set(res.1.unwrap());                           
                            log_errors.set(res.0);                                
                        },
                        Err(_err) => {}
                    }
                }
            }
        });
    };

    if let Some(t) = thread.get() {
        cx.render(rsx! {
            Page {
                div {
                    class: "flex justify-between mb-4",
                    h1 {
                         class: "text-2xl font-semibold pb-2",
                         "Thread"
                    }
                    button {
                        class: "bg-white text text-black font-medium align-middle justify-center pt-3 pb-3 pr-10 pl-10 disabled:opacity-20",
                        onclick: handle_click,                    
                        "Simulate Thread"
                    }
                    
                }
                table {
                    class: "min-w-full divide-y divide-gray-300",
                    tbody {
                        Row {
                            label: "address".to_string(),
                            value: thread_pubkey.to_string()
                        }
                        Row {
                            label: "authority".to_string(),
                            value: t.authority.to_string(),
                        }
                        Row {
                            label: "Fee".to_string(),
                            value: t.fee.to_string(),
                        }
                        Row {
                            label: "ID".to_string(),
                            value: t.id.to_string(),
                        }
                        Row {
                            label: "paused".to_string(),
                            value: t.paused.to_string(),
                        }
                    }
                }
            }
            if *show_modal.get() {
                rsx!{
                    div {
                        class: "min-w-[300px] lg:min-w-[800px] xl:min-w-[800px] 2xl:min-w-[800px] absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 my-4 p-10 bg-gray-200 rounded-lg flex flex-col shadow-md z-50",
                        div {
                            class: "flex justify-between items-center mb-3",
                            div {
                                h2 {
                                    class: "text-2xl text-black font-semibold font-header leading-5",
                                    "Simulate Thread"
                                }
                                p {
                                    class: "text-black font-light font-header leading-5 mt-2",
                                    "Program Logs"
                                }
                            }
                            div {
                                button {
                                    class: "cursor-pointer",
                                    onclick: move |_| show_modal.set(false),
                                    svg {
                                        view_box: "-1 0 48 48",
                                        class: "w-6 h-6",
                                        fill:"none",
                                        xmlns:"http://www.w3.org/2000/svg",
                                        path {
                                          d: "M2 2L36 36M2 36L36 2",
                                          stroke: "black",
                                          stroke_width: "3"
                                        }
                                    }
                                }
                            }
                        }
                        code {
                            class: "w-full h-auto flex flex-col p-3 rounded-md bg-gray-200 font-mono text-sm text-black break-all font-light",
                            for log in logs.get().iter() {
                              p {
                                    "{log}"
                                }   
                            }
                        }
                    }
                }
            }
        })
    } else {
        cx.render(rsx! {
            Page {
                div {
                    "loading..."
                }
            }
        })
    }
}

#[derive(PartialEq, Props)]
struct RowProps {
    label: String,
    value: String,
}

fn Row(cx: Scope<RowProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "flex justify-between px-3 text-sm border-b border-slate-800 hover:bg-slate-900 focus:bg-slate-900",
            id: cx.props.label.as_str(),
            div {
                class: "table-cell whitespace-nowrap px-3 py-4",
                "{cx.props.label}"
            }
            div {
                class: "table-cell whitespace-nowrap px-3 py-4",
                "{cx.props.value}"
            }
        }
    })
}
