use {
    dioxus::prelude::*,
    dioxus_router::Redirect,
};

pub fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        Redirect {
            to: "/accounts"
        }
    })
}
