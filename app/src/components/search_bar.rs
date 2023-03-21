use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_router::use_router;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::SearchState;

pub fn SearchPage(cx: Scope) -> Element {
    let search_state = use_shared_state::<SearchState>(cx).unwrap();
    let r_search_state = search_state.read();
    if r_search_state.is_searching {
        cx.render(rsx! {
            div {
                onclick: move |_| {
                    let mut w_search_state = search_state.write();
                    w_search_state.is_searching = false;
                    w_search_state.query = "".to_string();
                },
                class: "absolute top-0 left-0 w-screen h-screen backdrop-blur content-center flex flex-col",
                SearchBar{}
            }
        })
    } else {
        None
    }
}

pub fn SearchBar(cx: Scope) -> Element {
    let search_state = use_shared_state::<SearchState>(cx).unwrap();
    let query = &search_state.read().query;
    let router = use_router(cx);

    use_future(&cx, search_state, |_| {
        let search_state = search_state.clone();
        async move {
            gloo_timers::future::TimeoutFuture::new(50).await;
            let document = gloo_utils::document();
            if search_state.read().is_searching {
                if let Some(element) = document.get_element_by_id("search-bar") {
                    element.unchecked_into::<HtmlElement>().focus().ok();
                }
            }
        }
    });

    cx.render(rsx! {
        input {
            class: "border border-slate-700 rounded bg-slate-900 text-slate-100 p-4 w-full mx-auto max-w-3xl w-full mt-32",
            id: "search-bar",
            r#type: "text",
            placeholder: "Search",
            value: "{query}",
            oninput: move |e| {
                let query_str = e.value.clone().as_str().to_string();
                if query_str.ne(&String::from("/")) {
                    let mut w_search_state = search_state.write();
                    w_search_state.query = query_str;
                }
            },
            onclick: move |e| e.stop_propagation(),
            onkeydown: move |e| {
                if e.key() == Key::Enter {
                    let mut w_search_state = search_state.write();
                    let query = &w_search_state.query;

                    // TODO Parse the query into a pubkey address and navigate to the correct page.
                    if let Ok(address) = Pubkey::from_str(&query) {
                        router.navigate_to(&*format!("/accounts/{}", address.to_string()));
                        w_search_state.is_searching = false;
                        w_search_state.query = "".to_string();
                    } else {
                        // TODO Display "invalid address" error to user 
                        log::info!("Invalid address");
                    }
                }
            },
        }
    })
}

pub fn SearchButton(cx: Scope) -> Element {
    let search_state = use_shared_state::<SearchState>(cx).unwrap();
    cx.render(rsx! {
        button {
            class: "rounded-full bg-transparent text-slate-100 transition hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 p-3",
            onclick: move |_| {
                let mut w_search_state = search_state.write();
                w_search_state.is_searching = true;
            },
            svg {
                class: "w-6 h-6",
                fill: "none",
                view_box: "0 0 24 24",
                stroke_width: "1.5",
                stroke: "currentColor",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M21 21l-5.197-5.197m0 0A7.5 7.5 0 105.196 5.196a7.5 7.5 0 0010.607 10.607z",
                }
            }
        }
    })
}
