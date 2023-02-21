use dioxus::prelude::*;

pub fn DataPage(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Data" }
    })
}
