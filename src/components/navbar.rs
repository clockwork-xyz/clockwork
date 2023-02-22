use dioxus::prelude::*;
use dioxus_router::Link;

use crate::{components::ConnectButton, routes::RoutePath};

pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "fixed top-0 flex flex-row justify-between w-screen p-8",
            Logo {}
            ConnectButton {}
        }
    })
}

pub fn Logo(cx: Scope) -> Element {
    cx.render(rsx! {
        Link {
            to: RoutePath::Home.as_str(),
            img {
                src: "/img/CLOCKWORK.svg",
                class: "h-4"
            }
        }
    })
}
