use clockwork_sdk::state::{Thread, Trigger};
use clockwork_utils::pubkey::Abbreviated;
use dioxus::prelude::*;
use dioxus_router::{use_router, Link};
use solana_client_wasm::solana_sdk::account::Account;

use crate::{
    clockwork::get_threads,
    utils::{format_balance, format_timestamp},
};

pub fn ThreadsTable(cx: Scope) -> Element {
    let threads = use_state::<Vec<(Thread, Account)>>(&cx, || vec![]);

    use_future(&cx, (), |_| {
        let threads = threads.clone();
        async move { threads.set(get_threads().await) }
    });

    if threads.get().len() > 0 {
        cx.render(rsx! {
            table {
                class: "w-full",
                Header {}
                div {
                    class: "table-row-group",
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
    let cell_class = "table-cell font-medium py-2 first:pl-3";
    cx.render(rsx! {
        thead {
            class: "table-header-group",
            div {
                class: "table-row text-left text-sm text-slate-500",
                th {
                    class: cell_class,
                    scope: "col",
                    "Thread"
                }
                th {
                    class: cell_class,
                    scope: "col",
                    "Balance"
                }
                th {
                    class: cell_class,
                    scope: "col",
                    "Created at"
                }
                th {
                    class: cell_class,
                    scope: "col",
                    "ID"
                }
                th {
                    class: cell_class,
                    scope: "col",
                    "Last exec"
                }
                th {
                    class: cell_class,
                    scope: "col",
                    "Paused"
                }
                th {
                    class: cell_class,
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
    let router = use_router(cx);
    let thread = cx.props.thread.clone();
    let address = Thread::pubkey(thread.authority, thread.id.clone());
    let address_abbr = address.abbreviated();
    let balance = format_balance(cx.props.account.lamports);
    let created_at = format_timestamp(thread.created_at.unix_timestamp);
    let id = thread.id;
    let paused = thread.paused.to_string();
    let last_exec_at = match thread.exec_context {
        None => String::from("–"),
        Some(exec_context) => format!("{}", exec_context.last_exec_at)
    };
    let trigger = match thread.trigger {
        Trigger::Account {
            address,
            offset: _,
            size: _,
        } => address.abbreviated(),
        Trigger::Cron {
            schedule,
            skippable: _,
        } => schedule,
        Trigger::Immediate => "–".to_string(),
    };
    let cell_class = "table-cell whitespace-nowrap first:pl-3 first:rounded-tl first:rounded-bl last:rounded-tr last:rounded-br py-2";
    cx.render(rsx! {
        Link {
            class: "table-row font-mono text-sm transition hover:cursor-pointer hover:bg-slate-800 active:bg-slate-100 active:text-slate-900",
            to: "/programs/threads/{address}",
            id: cx.props.elem_id.as_str(),
            div {
                class: cell_class,
                "{address_abbr}"
            }
            div {
                class: cell_class,
                "{balance}"
            }
            div {
                class: cell_class,
                "{created_at}"
            }
            div {
                class: cell_class,
                "{id}"
            }
            div {
                class: cell_class,
                "{last_exec_at}"
            }
            div {
                class: cell_class,
                "{paused}"
            }
            div {
                class: cell_class,
                "{trigger}"
            }
        }
    })
}
