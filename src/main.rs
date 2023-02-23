#![allow(non_snake_case)]
mod components;
mod hot_keys;
mod pages;
mod pyth;

use components::*;
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use hot_keys::Goto;
use pages::*;
use wasm_logger;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w-screen h-screen flex flex-col justify-start",
            Router {
                Goto {}
                Navbar {}
                Clock {}
                Route { to: "/", HomePage{} }
                Route { to: "/data", DataPage{} }
                Route { to: "/files", FilesPage{} }
                Route { to: "/price_feed/:address", PriceFeedPage{} }
                Route { to: "/programs", ProgramsPage{} }
                Route { to: "", NotFoundPage{} }
            }
        }
    })
}
