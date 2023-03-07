use crate::clockwork::get_threads;
use chrono::{Duration, TimeZone, Utc};
use clockwork_sdk::state::Thread;
use dioxus::prelude::*;
use dotenv_codegen::dotenv;
use solana_client_wasm::WasmClient;

pub fn ThreadsTable(cx: Scope) -> Element {
    let threads = use_state::<Vec<Thread>>(&cx, || vec![]);

    use_future(&cx, (), |_| {
        let threads = threads.clone();
        async move { threads.set(get_threads().await) }
    });

    cx.render(rsx! {
        div {
            h1 {
             class: "text-2xl font-semibold pb-2",
             "Threads"
            }
            Header {}
            for thread in threads.get().iter() {
                Row {
                    elem_id: thread.id.as_str(),
                    thread: thread.clone(),
                }
            }
        }
    })
}

fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full flex flex-row justify-between py-3 border-b border-slate-800 font-medium text-sm text-slate-600",
            p {
                "Address"
            }
            p {
                "last exec at"
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct RowProps<'a> {
    thread: Thread,
    elem_id: &'a str,
}

fn Row<'a>(cx: Scope<'a, RowProps<'a>>) -> Element {
    let thread = cx.props.thread.clone();
    let thread_pubkey = Thread::pubkey(thread.authority, thread.id.clone()).to_string();
    let last_exec_at = use_state(&cx, || String::from(""));

    use_future(&cx, (), |_| {
        let last_exec_at = last_exec_at.clone();
        async move { last_exec_at.set(time_duration(thread).await) }
    });

    cx.render(rsx! {
        a {
            href: "/thread/{thread_pubkey}",
            class: "w-full flex flex-row justify-between py-3 border-b border-slate-800 hover:bg-slate-900 focus:bg-slate-900",
            id: cx.props.elem_id,
            p {
                "{thread_pubkey}"
            }
            p {
                "{last_exec_at}"
            }
        }
    })
}

async fn time_duration(thread: Thread) -> String {
    let client = WasmClient::new("http://74.118.139.244:8899");
    // const HELIUS_API_KEY: &str = dotenv!("HELIUS_API_KEY");
    // let url = format!("https://rpc.helius.xyz/?api-key={}", HELIUS_API_KEY);
    // let helius_rpc_endpoint = url.as_str();
    // let client = WasmClient::new(helius_rpc_endpoint);

    if thread.exec_context.is_none() {
        return String::from("");
    }

    let last_exec_at = thread.exec_context.unwrap().last_exec_at;
    let lea_ts = client.get_block_time(last_exec_at).await.unwrap();
    let now_ts = Utc::now().timestamp();
    let diff_ts = now_ts.checked_sub(lea_ts).unwrap();
    let diff = Duration::seconds(diff_ts);

    if diff.num_weeks() > 0 {
        if diff.num_weeks() == 1 {
            return String::from(format!("{} week ago", diff.num_weeks()));
        }
        return String::from(format!("{} weeks ago", diff.num_weeks()));
    } else if diff.num_days() > 0 {
        if diff.num_days() == 1 {
            return String::from(format!("{} day ago", diff.num_days()));
        }
        return String::from(format!("{} days ago", diff.num_days()));
    } else if diff.num_hours() > 0 {
        if diff.num_hours() == 1 {
            return String::from(format!("{} hour ago", diff.num_hours()));
        }
        return String::from(format!("{} hours ago", diff.num_hours()));
    } else if diff.num_minutes() > 0 {
        if diff.num_minutes() == 1 {
            return String::from(format!("{} minute ago", diff.num_minutes()));
        }
        return String::from(format!("{} minutes ago", diff.num_minutes()));
    } else if diff.num_seconds() > 30 {
        return String::from(format!("{} seconds ago", diff.num_seconds()));
    } else {
        return String::from("a few seconds ago");
    }
}
