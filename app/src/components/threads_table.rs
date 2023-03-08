use crate::{
    clockwork::get_threads,
    utils::{format_balance, format_timestamp},
};
use clockwork_sdk::state::{Thread, Trigger};
use dioxus::prelude::*;
use dioxus_router::Link;
use solana_client_wasm::solana_sdk::account::Account;

pub fn ThreadsTable(cx: Scope) -> Element {
    let threads = use_state::<Vec<(Thread, Account)>>(&cx, || vec![]);

    use_future(&cx, (), |_| {
        let threads = threads.clone();
        async move { threads.set(get_threads().await) }
    });

    if threads.get().len() > 0 {
        cx.render(rsx! {
            // h1 {
            //  class: "text-2xl font-semibold pb-2",
            //  "Threads"
            // }
            table {
                class: "min-w-full divide-y divide-gray-300",
                Header {}
                tbody {
                    for (i, thread) in threads.get().iter().enumerate() {
                        Row {
                            thread: thread.0.clone(),
                            account: thread.1.clone(),
                            elem_id: format!("list-item-{}", i),
                        }
                    }
                }
            }
        })
    } else {
        cx.render(rsx! {
            div {
                "Loading..."
            }
        })
    }
}

fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        thead {
            tr {
                th {
                    class: "py-3.5 text-left text-sm font-semibold sm:pl-3",
                    scope: "col",
                    "Address"
                }
                th {
                    class: "py-3.5 text-left text-sm font-semibold sm:pl-3",
                    scope: "col",
                    "Balance"
                }
                th {
                    class: "py-3.5 text-left text-sm font-semibold sm:pl-3",
                    scope: "col",
                    "Created at"
                }
                th {
                    class: "py-3.5 text-left text-sm font-semibold sm:pl-3",
                    scope: "col",
                    "ID"
                }
                th {
                    class: "py-3.5 text-left text-sm font-semibold sm:pl-3",
                    scope: "col",
                    "Paused"
                }
                th {
                    class: "py-3.5 text-left text-sm font-semibold sm:pl-3",
                    scope: "col",
                    "Trigger"
                }
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct RowProps {
    thread: Thread,
    account: Account,
    elem_id: String,
}

fn Row(cx: Scope<RowProps>) -> Element {
    let thread = cx.props.thread.clone();
    let thread_pubkey = Thread::pubkey(thread.authority, thread.id.clone()).to_string();
    let balance = format_balance(cx.props.account.lamports);
    let created_at = format_timestamp(thread.created_at.unix_timestamp);
    let id = thread.id;
    let paused = thread.paused.to_string();
    let trigger = match thread.trigger {
        Trigger::Account {
            address: _,
            offset: _,
            size: _,
        } => "Account".to_string(),
        Trigger::Cron {
            schedule: _,
            skippable: _,
        } => "Cron".to_string(),
        Trigger::Immediate => "Immediate".to_string(),
    };
    cx.render(rsx! {
        tr {
             class: "px-3 text-sm border-b border-slate-800 hover:bg-slate-900 hover:cursor-pointer focus:bg-slate-900",
             id: cx.props.elem_id.as_str(),
             td {
                 class: "whitespace-nowrap px-3 py-4",
                 "{thread_pubkey}"
             }
             td {
                 class: "whitespace-nowrap px-3 py-4",
                 "{balance}"
             }
             td {
                 class: "whitespace-nowrap px-3 py-4",
                 "{created_at}"
             }
             td {
                 class: "whitespace-nowrap px-3 py-4",
                 "{id}"
             }
             td {
                 class: "whitespace-nowrap px-3 py-4",
                 "{paused}"
             }
             td {
                 class: "whitespace-nowrap px-3 py-4",
                 "{trigger}"
             }
        }
    })
}
