use dioxus::prelude::*;

use crate::components::{BlocksTable, MarketsTable};

use super::Page;

pub fn BlocksPage(cx: Scope) -> Element {
    cx.render(rsx! {
        Page {
            div {
                class: "flex flex-col space-y-16",
                BlocksTable {}
                MarketsTable {}
            }
        }
    })
}
