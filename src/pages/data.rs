use dioxus::prelude::*;

use crate::components::MarketsTable;

use super::Page;

pub fn DataPage(cx: Scope) -> Element {
    cx.render(rsx! {
        Page {
            h1 {
                class: "text-2xl font-semibold",
                "Markets"
            }
            MarketsTable {}
        }
    })
}
