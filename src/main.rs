#![allow(non_snake_case)]
mod components;
mod routes;

use components::*;
use dioxus::prelude::*;
use dioxus_router::{Route, Router};
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
                Route { to: RoutePath::Home.as_str(), HomePage{} }
                Route { to: RoutePath::Data.as_str(), DataPage{} }
                Route { to: RoutePath::Files.as_str(), FilesPage{} }
                Route { to: RoutePath::Threads.as_str(), ThreadsPage{} }
                Route { to: RoutePath::NotFound.as_str(), NotFoundPage{} }
            }
        }
    })
}

fn HomePage(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Home" }
    })
}

fn DataPage(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Data" }
    })
}

fn FilesPage(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Files" }
    })
}

fn ThreadsPage(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Threads" }
    })
}

fn NotFoundPage(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Not found" }
    })
}
