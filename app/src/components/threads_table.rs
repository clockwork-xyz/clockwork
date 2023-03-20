use std::str::FromStr;

use chrono::{DateTime, NaiveDateTime, Utc};
use clockwork_thread_program_v2::state::{Thread, Trigger, TriggerContext, VersionedThread};
use clockwork_utils::pubkey::Abbreviated;
use dioxus::prelude::*;
use dioxus_router::{use_router, Link};
use solana_client_wasm::solana_sdk::account::Account;

use crate::{
    clockwork::get_threads,
    utils::{format_balance, format_timestamp},
};

pub fn ThreadsTable(cx: Scope) -> Element {
    let threads = use_state::<Vec<(VersionedThread, Account)>>(&cx, || vec![]);

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
                    "Status"
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
    thread: VersionedThread,
    account: Account,
    elem_id: String,
}

fn Row(cx: Scope<RowProps>) -> Element {
    let router = use_router(cx);
    let thread = cx.props.thread.clone();
    let address = thread.pubkey(); // Thread::pubkey(thread.authority(), thread.id().clone());
    let address_abbr = address.abbreviated();
    let balance = format_balance(cx.props.account.lamports, true);
    let created_at = format_timestamp(thread.created_at().unix_timestamp);
    let id = String::from_utf8(thread.id()).unwrap();
    let paused = thread.paused().to_string();
    let last_exec_at = match thread.exec_context() {
        None => String::from("–"),
        Some(exec_context) => format!("{}", exec_context.last_exec_at),
    };
    let trigger = match thread.trigger() {
        Trigger::Account {
            address,
            offset: _,
            size: _,
        } => address.abbreviated(),
        Trigger::Cron {
            schedule,
            skippable: _,
        } => {
            let reference_timestamp = match thread.exec_context().clone() {
                None => thread.created_at().unix_timestamp,
                Some(exec_context) => match exec_context.trigger_context {
                    TriggerContext::Cron { started_at } => started_at,
                    _ => 0,
                },
            };
            next_timestamp(reference_timestamp, schedule)
                .map_or("–".to_string(), |v| format_timestamp(v))
        }
        Trigger::Now => "–".to_string(),
        Trigger::Slot { slot } => slot.to_string(),
        Trigger::Epoch { epoch } => epoch.to_string(),
        Trigger::Timestamp { unix_ts } => unix_ts.to_string(),
    };
    enum ThreadStatus {
        Healthy,
        Unhealthy,
        Unknown,
    }
    let status = match thread.trigger() {
        Trigger::Account {
            address,
            offset: _,
            size: _,
        } => ThreadStatus::Unknown,
        Trigger::Cron {
            schedule,
            skippable: _,
        } => {
            let reference_timestamp = match thread.exec_context().clone() {
                None => thread.created_at().unix_timestamp,
                Some(exec_context) => match exec_context.trigger_context {
                    TriggerContext::Cron { started_at } => started_at,
                    _ => 0,
                },
            };
            if let Some(target_ts) = next_timestamp(reference_timestamp, schedule) {
                // TODO Compare the target timestamp to the current timestamp. If this thread should have fired a while ago, it is "unhealthy".
                ThreadStatus::Healthy
            } else {
                ThreadStatus::Healthy
            }
        }
        Trigger::Now => ThreadStatus::Unhealthy,
        Trigger::Slot { slot: _ } => ThreadStatus::Unknown,
        Trigger::Epoch { epoch: _ } => ThreadStatus::Unknown,
        Trigger::Timestamp { unix_ts: _ } => ThreadStatus::Unknown,
    };
    let status_class = match status {
        ThreadStatus::Healthy => "w-3 h-3 bg-green-500 rounded-full ml-4",
        ThreadStatus::Unhealthy => "w-3 h-3 bg-red-500 rounded-full ml-4",
        ThreadStatus::Unknown =>"w-3 h-3 bg-slate-500 rounded-full ml-4",
    };
    let cell_class = "table-cell whitespace-nowrap first:pl-3 first:rounded-tl first:rounded-bl last:rounded-tr last:rounded-br py-2";
    cx.render(rsx! {
        Link {
            class: "table-row font-mono text-sm items-start transition hover:cursor-pointer hover:bg-slate-800 active:bg-slate-100 active:text-slate-900",
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
                div {
                    class: status_class, 
                }
            }
            div {
                class: cell_class,
                "{trigger}"
            }
        }
    })
}

fn next_timestamp(after: i64, schedule: String) -> Option<i64> {
    clockwork_cron::Schedule::from_str(&schedule)
        .unwrap()
        .next_after(&DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp_opt(after, 0).unwrap(),
            Utc,
        ))
        .take()
        .map(|datetime| datetime.timestamp())
}
