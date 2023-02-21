use dioxus::prelude::*;

pub fn FilesPage(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Files" }
    })
}
