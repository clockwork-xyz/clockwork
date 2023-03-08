use dioxus::prelude::*;

use crate::components::MarketsTable;

use super::Page;

pub fn DataPage(cx: Scope) -> Element {
    cx.render(rsx! {
        Page {
            MarketsTable {}
        }
    })
}
