use super::Page;
use clockwork_sdk::state::Thread;
use dioxus::prelude::*;
use dioxus_router::use_route;

pub fn ThreadPage(cx: Scope) -> Element {
    let route = use_route(cx);
    let thread = use_state::<Option<Thread>>(cx, || None);

    let thread_pk: String = match route.segment("address") {
        Some(pk) => pk.to_string(),
        None => "An unknown error occured".to_string(),
    };

    log::info!("thread pubkey: {}", thread_pk);

    cx.render(rsx! {
        Page {
            h1 {
                class: "text-2xl font-semibold",
                "Thread"
            }
        }
    })
}
