use dioxus::prelude::*;
use dioxus_router::Link;

use crate::components::ConnectButton;

pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-row justify-between w-screen p-8",
            Logo {}
            ConnectButton {}
        }
    })
}

pub fn Logo(cx: Scope) -> Element {
    cx.render(rsx! {
        Link {
            to: "/",
            class: "flex items-center w-40",
            img {
                src: "/img/CLOCKWORK.svg",
            }
        }
    })
}
