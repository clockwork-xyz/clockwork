#![allow(non_snake_case)]
mod components;
mod pages;
mod routes;

use components::*;
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
use pages::*;
use wasm_logger;

use crate::routes::RoutePath;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w-screen h-screen flex items-center justify-center",
            Router {
                Navbar {}
                Sidebar {}
                Clock {}
                Route { to: RoutePath::Home.as_str(), HomePage{} }
                Route { to: RoutePath::Data.as_str(), DataPage{} }
                Route { to: RoutePath::Files.as_str(), FilesPage{} }
                Route { to: RoutePath::Threads.as_str(), ThreadsPage{} }
                Route { to: RoutePath::NotFound.as_str(), NotFoundPage{} }
            }
        }
    })
}
