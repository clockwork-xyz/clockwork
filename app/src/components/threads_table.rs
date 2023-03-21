use crate::{
    clockwork::get_threads,
    hooks::use_pagination,
    utils::{format_balance, format_timestamp},
};
use clockwork_sdk::state::{Thread, Trigger};
use clockwork_thread_program_v2::state::VersionedThread;
use dioxus::prelude::*;
use dioxus_router::Link;
use solana_client_wasm::solana_sdk::account::Account;

pub fn ThreadsTable(cx: Scope) -> Element {
    let paginated_threads = use_pagination::<(Thread, Account)>(&cx, 15, || vec![]);

    use_future(&cx, (), |_| {
        let paginated_threads = paginated_threads.clone();
        async move { paginated_threads.set(get_threads().await) }
    });

    if let Some(threads) = paginated_threads.get() {
        cx.render(rsx! {
            table {
                class: "w-full divide-y divide-slate-800",
                Header {}
                for (i, thread) in threads.iter().enumerate() {
                    Row {
                        thread: thread.0.clone(),
                        account: thread.1.clone(),
                        elem_id: format!("list-item-{}", i),
                    }
                }
            }
            div {
                class: "flex items-center justify-center space-x-2",
                button {
                    class: "py-2 px-6 dark:text-white-200 hover:bg-slate-600 text-black-100 text-sm font-medium rounded-lg",
                    onclick: move |_| { paginated_threads.prev_page() },
                    "←"
                }
                p {
                    class: "text-sm text-gray-100",
                    "{paginated_threads.current_page() + 1}"
                }
                button {
                    class: "py-2 px-6 dark:text-white-200 hover:bg-slate-600 text-black-100 text-sm font-medium rounded-lg",
                    onclick: move |_| { paginated_threads.next_page() },
                    "→"
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
                class: "text-left text-sm text-slate-500",
                th {
                    class: "py-3 px-3 font-medium",
                    scope: "col",
                    "Thread"
                }
                th {
                    class: "py-3 px-3 font-medium",
                    scope: "col",
                    "Balance"
                }
                th {
                    class: "py-3 px-3 font-medium",
                    scope: "col",
                    "Created at"
                }
                th {
                    class: "py-3 px-3 font-medium",
                    scope: "col",
                    "ID"
                }
                th {
                    class: "py-3 px-3 font-medium",
                    scope: "col",
                    "Paused"
                }
                th {
                    class: "py-3 px-3 font-medium",
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
        Link {
            class: "table-row px-3 text-base hover:bg-slate-100 hover:text-slate-900",
            to: "/thread/{thread_pubkey}",
            id: cx.props.elem_id.as_str(),
            div {
                class: "table-cell whitespace-nowrap px-4 py-4",
                "{thread_pubkey}"
            }
            div {
                class: "table-cell whitespace-nowrap px-4 py-4",
                "{balance}"
            }
            div {
                class: "table-cell whitespace-nowrap px-4 py-4",
                "{created_at}"
            }
            div {
                class: "table-cell whitespace-nowrap px-4 py-4",
                "{id}"
            }
            div {
                class: "table-cell whitespace-nowrap px-4 py-4",
                "{paused}"
            }
            div {
                class: "table-cell whitespace-nowrap px-4 py-4",
                "{trigger}"
            }
        }
    })
}
