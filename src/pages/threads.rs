use dioxus::prelude::*;

pub fn ThreadsPage(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Threads" }
    })
}
