use std::str::FromStr;
use std::convert::From;
use anchor_lang::solana_program::pubkey::Pubkey; 
use chrono::{DateTime, NaiveDateTime, Utc};
use dioxus::prelude::*;
use dioxus_router::use_route;
use clockwork_thread_program_v2::state::{VersionedThread, Trigger, VersionedID};
use solana_client_wasm::solana_sdk::transaction::TransactionError;

use crate::{clockwork::{get_thread, simulate_thread}, utils::format_timestamp};

use super::Page;

pub fn ThreadPage(cx: Scope) -> Element {
    let route = use_route(cx);
    let thread = use_state::<Option<VersionedThread>>(cx, || None);

    use_future(&cx, (), |_| {
        let thread = thread.clone();
        let thread_pubkey = Pubkey::from_str(route.last_segment().unwrap()).unwrap();
        async move { 
            let t = get_thread(thread_pubkey).await;
            thread.set(Some(t.clone()));
        }
    });
    
    if let Some(t) = thread.get() {
        cx.render(rsx! {
            Page {
                div {
                    class: "flex flex-col space-y-16",
                    div {
                        class: "flex flex-col justify-between",
                        h1 {
                             class: "text-2xl font-semibold mb-6",
                             "Thread"
                        }
                        ThreadInfoTable { thread: t.clone() }
                    }
                    SimulationLogs { thread: t.clone() }
                }
            }
        })
    } else {
        cx.render(rsx! {
            Page {
                div {
                    "Loading..."
                }
            }
        })
    }
}

#[derive(PartialEq, Props)]
struct ThreadInfoTableProps{
    thread: VersionedThread,
}

fn ThreadInfoTable(cx: Scope<ThreadInfoTableProps>) -> Element {
    let thread = &cx.props.thread;
    let address = thread.pubkey();
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
    };

    let thread_id = match thread.id() {
        VersionedID::String(id) => id,
        VersionedID::Vec(_) => String::from("Vec<u8>"),
    };

    cx.render(rsx! {
        table {
            class: "w-full divide-y divide-slate-800",
            tbody {
                Row {
                    label: "Address".to_string(),
                    value: address.to_string()
                }
                Row {
                    label: "Thread Program".to_string(),
                    value: thread.program_id().to_string()
                }
                Row {
                    label: "Authority".to_string(),
                    value: thread.authority().to_string(),
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
                    value: thread_id,
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
                class: "table-cell whitespace-nowrap px-4 py-4 text-sm text-slate-500",
                "{cx.props.label}"
            }
            div {
                class: "table-cell whitespace-nowrap px-4 py-4 text-base text-slate-100",
                "{cx.props.value}"
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct SimulationLogsProps{
    thread: VersionedThread,
}


fn SimulationLogs(cx: Scope<SimulationLogsProps>) -> Element {
    let logs = use_state::<Vec<String>>(cx, || vec![]);
    let log_errors = use_state::<Option<TransactionError>>(cx, || None);

    let tx_err = if log_errors.get().is_some() {
        let mut hex: Option<String> = None;
        let raw_text = log_errors.as_ref().unwrap().to_string();
        let mut error_string = String::new();
        for word in raw_text.split(" ").collect::<Vec<_>>().iter() {
            hex = match word.strip_prefix("0x") {
                Some(d) => Some(String::from(d.to_string())),
                None => {
                    let w = String::from(format!("{} ", word));
                    error_string.push_str(w.as_str());
                    None
                },
            }
        }

        if hex.is_some() {
            let dec = i64::from_str_radix(hex.unwrap().as_str(), 16);
            String::from(format!("{}{}", error_string, dec.unwrap()))
        } else {
            String::from(format!("{}", error_string))
        }
        
    } else {
        String::from("")
    };

    use_future(&cx, (), |_| {
        let thread = cx.props.thread.clone();
        let logs = logs.clone();
        let log_errors = log_errors.clone();
        async move { 
            match simulate_thread(thread.to_owned(), thread.pubkey()).await {
                Ok(res) => {
                    logs.set(res.1.unwrap());                           
                    log_errors.set(res.0);                                
                },
                Err(_err) => {}
            }
        }
    });


    cx.render(rsx! {
        div {
            class: "flex flex-col mb-6",
            h1 {
                class: "text-2xl text-slate-100 font-semibold font-header",
                "Simulation Logs"
            }
            code {
                class: "w-full h-auto flex flex-col px-4 py-4 mt-2 font-mono text-base text-slate-100 break-all rounded-md bg-red-500",
                "{tx_err}"
            }
            code {
                class: "w-full h-auto flex flex-col px-4 py-2 font-mono text-base text-slate-100 break-all",
                for log in logs.get().iter() {
                    p {
                        "{log}"
                    }   
                }
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
