#![allow(non_snake_case)]
mod clockwork;
mod components;
mod context;
mod hooks;
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

#[derive(Debug)]
pub struct SearchState {
    pub active: bool,
    pub busy: bool,
    pub query: String,
    pub results: Vec<SearchResult>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SearchResult {
    pub title: String,
    pub route: String,
}

impl PartialEq for SearchState {
    fn eq(&self, other: &Self) -> bool {
        self.active.eq(&other.active)
    }
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || SearchState {
        active: false,
        busy: false,
        query: String::new(),
        results: vec![],
    });
    use_shared_state_provider(cx, || User::default());

    cx.render(rsx! {
        div {
            class: "w-screen h-screen flex flex-col justify-start",
            Router {
                HotKeys {}
                Navbar {}
                Route { to: "/", HomePage{} }
                Route { to: "/accounts", AccountsPage{} }
                Route { to: "/accounts/:address", AccountPage{} }
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
