use dioxus::prelude::*;

use super::Page;

pub fn MarketPage(cx: Scope) -> Element {
    cx.render(rsx! {
        Page {
            h1 {
                class: "text-2xl font-semibold",
                "Market"
            }
        }
    })
}
