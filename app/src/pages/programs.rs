use dioxus::prelude::*;

use crate::components::ThreadsTable;

use super::Page;

pub fn ProgramsPage(cx: Scope) -> Element {
    cx.render(rsx! {
        Page {
            h1 {
                class: "text-2xl font-semibold mb-6",
                "Programs"
            }
            ThreadsTable {}
        }
    })
}
