use dioxus::prelude::*;

use crate::components::MarketsTable;

use super::Page;

pub fn AccountsPage(cx: Scope) -> Element {
    cx.render(rsx! {
        Page {
            div {
                class: "flex flex-col space-y-16",
                // BlocksTable {}
                MarketsTable {}
            }
        }
    })
}
