#![allow(non_snake_case)]
mod clockwork;
mod components;
mod context;
mod hot_keys;
mod pages;
mod pyth;
mod utils;

use components::*;
use context::*;
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use hot_keys::HotKeys;
use pages::*;
use wasm_logger;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}

pub struct SearchState {
    pub is_searching: bool,
    pub query: String,
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || SearchState {
        is_searching: false,
        query: String::new(),
    });
    use_shared_state_provider(cx, || User {
        pubkey: None,
        account: None,
    });

    cx.render(rsx! {
        div {
            class: "w-screen h-screen flex flex-col justify-start",
            Router {
                HotKeys {}
                Navbar {}
                Route { to: "/", HomePage{} }
                Route { to: "/accounts", AccountsPage{} }
                Route { to: "/accounts/markets/:address", MarketPage{} }
                Route { to: "/files", FilesPage{} }
                Route { to: "/keys", KeysPage{} }
                Route { to: "/keys/new", NewKeyPage{} }
                Route { to: "/programs", ProgramsPage{} }
                Route { to: "/programs/threads/:address", ThreadPage {} }
                Route { to: "/settings", SettingsPage {} }
                Route { to: "", NotFoundPage{} }
                SearchPage {}
            }
        }
    })
}
