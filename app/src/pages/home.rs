use dioxus::prelude::*;

pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Home" }
    })
}
