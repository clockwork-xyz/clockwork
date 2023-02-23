use dioxus::prelude::*;

use crate::components::PriceFeedsTable;

use super::Page;

pub fn DataPage(cx: Scope) -> Element {
    cx.render(rsx! {
        Page {
            h1 {
                class: "text-2xl font-semibold",
                "Data"
            }
            PriceFeedsTable {}
        }
    })
}
