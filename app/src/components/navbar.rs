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
                SettingsButton {}
                // NetworkSelector {}
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
        format_balance(account.lamports, true)
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

pub fn SettingsButton(cx: Scope) -> Element {
    cx.render(rsx! {
        Link {
            class: "rounded-full bg-transparent text-slate-100 transition hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 p-3",
            to: "/settings",
            svg {
                class: "w-6 h-6",
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "1.5",
                stroke: "currentColor",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M9.594 3.94c.09-.542.56-.94 1.11-.94h2.593c.55 0 1.02.398 1.11.94l.213 1.281c.063.374.313.686.645.87.074.04.147.083.22.127.324.196.72.257 1.075.124l1.217-.456a1.125 1.125 0 011.37.49l1.296 2.247a1.125 1.125 0 01-.26 1.431l-1.003.827c-.293.24-.438.613-.431.992a6.759 6.759 0 010 .255c-.007.378.138.75.43.99l1.005.828c.424.35.534.954.26 1.43l-1.298 2.247a1.125 1.125 0 01-1.369.491l-1.217-.456c-.355-.133-.75-.072-1.076.124a6.57 6.57 0 01-.22.128c-.331.183-.581.495-.644.869l-.213 1.28c-.09.543-.56.941-1.11.941h-2.594c-.55 0-1.02-.398-1.11-.94l-.213-1.281c-.062-.374-.312-.686-.644-.87a6.52 6.52 0 01-.22-.127c-.325-.196-.72-.257-1.076-.124l-1.217.456a1.125 1.125 0 01-1.369-.49l-1.297-2.247a1.125 1.125 0 01.26-1.431l1.004-.827c.292-.24.437-.613.43-.992a6.932 6.932 0 010-.255c.007-.378-.138-.75-.43-.99l-1.004-.828a1.125 1.125 0 01-.26-1.43l1.297-2.247a1.125 1.125 0 011.37-.491l1.216.456c.356.133.751.072 1.076-.124.072-.044.146-.087.22-.128.332-.183.582-.495.644-.869l.214-1.281z"
                }
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                }
            }
        }
    })
}
