use dioxus::prelude::*;

use super::Page;

pub fn PriceFeedPage(cx: Scope) -> Element {
    cx.render(rsx! {
        Page {
            h1 {
                class: "text-2xl font-semibold",
                "Price Feed"
            }
        }
    })
}
