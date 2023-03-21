use clockwork_thread_program_v2::state::{Trigger, VersionedThread};
use dioxus::prelude::*;
use solana_client_wasm::solana_sdk::account::Account;

use crate::utils::{format_balance, format_timestamp};

#[derive(PartialEq, Props)]
pub struct ThreadInfoTableProps {
    account: Account,
    thread: VersionedThread,
}

pub fn ThreadInfoTable(cx: Scope<ThreadInfoTableProps>) -> Element {
    let thread = &cx.props.thread;
    let address = thread.pubkey();
    let balance = format_balance(cx.props.account.lamports, false);
    let created_at = format_timestamp(thread.created_at().unix_timestamp);
    let trigger = match thread.trigger().clone() {
        Trigger::Account {
            address,
            offset: _,
            size: _,
        } => address.to_string(),
        Trigger::Cron {
            schedule,
            skippable: _,
        } => schedule.clone(),
        Trigger::Now => "Now".to_string(),
        Trigger::Slot { slot } => slot.to_string(),
        Trigger::Epoch { epoch } => epoch.to_string(),
        Trigger::Timestamp { unix_ts } => unix_ts.to_string(),
    };

    let id = String::from_utf8(thread.id()).unwrap();

    cx.render(rsx! {
        table {
            class: "w-full divide-y divide-slate-800",
            tbody {
                Row {
                    label: "Address".to_string(),
                    value: address.to_string()
                }
                Row {
                    label: "Authority".to_string(),
                    value: thread.authority().to_string(),
                }
                Row {
                    label: "Balance".to_string(),
                    value: balance,
                }
                Row {
                    label: "Created at".to_string(),
                    value: created_at,
                }
                // Row {
                //     label: "Fee".to_string(),
                //     value: thread.fee.to_string(),
                // }
                Row {
                    label: "ID".to_string(),
                    value: id,
                }
                Row {
                    label: "Paused".to_string(),
                    value: thread.paused().to_string(),
                }
                Row {
                    label: "Trigger".to_string(),
                    value: trigger,
                }
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct RowProps {
    label: String,
    value: String,
}

fn Row(cx: Scope<RowProps>) -> Element {
    cx.render(rsx! {
        div {
            class: "flex justify-between",
            id: cx.props.label.as_str(),
            div {
                class: "table-cell whitespace-nowrap px-2 py-2 text-sm text-slate-500",
                "{cx.props.label}"
            }
            div {
                class: "table-cell whitespace-nowrap px-2 py-2 text-sm font-mono text-slate-100",
                "{cx.props.value}"
            }
        }
    })
}
