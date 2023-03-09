use dioxus::prelude::*;
use dioxus_router::Link;

use super::Page;

pub fn SecretsPage(cx: Scope) -> Element {
    cx.render(rsx! {
        Page {
            div {
                class: "flex flex-row justify-between",
                h1 {
                    class: "text-2xl font-semibold",
                    "Secrets"
                }
                Link {
                    to: "/secrets/new"
                    class: "bg-white text-slate-900 font-semibold py-2 px-4 hover:bg-slate-200 transition",
                    "New secret"
                }
            }
        }
    })
}