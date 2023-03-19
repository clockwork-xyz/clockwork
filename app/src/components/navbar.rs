use dioxus::prelude::*;
use dioxus_router::Link;

use crate::{
    components::{ConnectButton, SearchButton},
    context::User,
    utils::format_balance,
};

pub fn Navbar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-row justify-between w-screen p-8",
            Logo {}
            div {
                class: "flex items-center space-x-4",
                SearchButton {}
                Balance {}
                ConnectButton {}
            }
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

fn Balance(cx: Scope) -> Element {
    let user_context = use_shared_state::<User>(cx).unwrap();

    let user_balance = if let Some(account) = &user_context.read().account {
        format_balance(account.lamports)
    } else {
        String::from("")
    };

    cx.render(rsx! {
        div {
            class: "text-lg",
            user_balance
        }
    })
}
