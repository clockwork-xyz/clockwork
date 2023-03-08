use dioxus::prelude::*;

pub fn NotFoundPage(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Not found" }
    })
}
