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

fn App(cx: Scope) -> Element {
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
                Clock {}
                Route { to: "/", HomePage{} }
                Route { to: "/data", DataPage{} }
                Route { to: "/data/market/:address", MarketPage{} }
                Route { to: "/thread/:address", ThreadPage {} }
                Route { to: "/files", FilesPage{} }
                Route { to: "/programs", ProgramsPage{} }
                Route { to: "/secrets", SecretsPage{} }
                Route { to: "/secrets/new", NewSecretPage{} }
                Route { to: "", NotFoundPage{} }
            }
        }
    })
}
