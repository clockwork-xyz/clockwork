use std::str::FromStr;

use anchor_lang::{prelude::Pubkey, Discriminator};
use clockwork_utils::pubkey::Abbreviated;
use dioxus::{html::input_data::keyboard_types::Key, prelude::*};
use dioxus_router::{use_router, Link};
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::{context::Client, SearchResult, SearchState};

pub fn SearchPage(cx: Scope) -> Element {
    let search_state = use_shared_state::<SearchState>(cx).unwrap();
    if search_state.read().active {
        cx.render(rsx! {
            div {
                onclick: move |_| {
                    let mut w_search_state = search_state.write();
                    w_search_state.active = false;
                },
                class: "absolute top-0 left-0 w-screen h-screen backdrop-opacity-10 bg-white/10 transition content-center flex flex-col",
                div {
                    class: "max-w-3xl w-full mx-auto mt-40 bg-[#0e0e10] p-1 flex flex-col rounded drop-shadow-md",
                    SearchBar {}
                    SearchResults {}
                }
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

    // Move the focus to the search bar.
    // autofocus property on input is having issues: https://github.com/DioxusLabs/dioxus/issues/725
    use_effect(&cx, (), |_| async move {
        gloo_timers::future::TimeoutFuture::new(50).await;
        let document = gloo_utils::document();
        if let Some(element) = document.get_element_by_id("search-bar") {
            element.unchecked_into::<HtmlElement>().focus().ok();
        }
    });

    cx.render(rsx! {
        input {
            class: "rounded bg-[#0e0e10] text-slate-100 p-5 w-full focus:ring-0 focus:outline-0 text-base",
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

                    // TODO Select navigation desination from the search results.
                    if let Ok(address) = Pubkey::from_str(&query) {
                        router.navigate_to(&*format!("/accounts/{}", address.to_string()));
                        w_search_state.active = false;
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

pub fn SearchResults(cx: Scope) -> Element {
    let search_state = use_shared_state::<SearchState>(cx).unwrap();
    let client_context = use_shared_state::<Client>(cx).unwrap();
    let query = &search_state.read().query;

    // Search for search results.
    let results = use_future(&cx, query, |_| {
        let query = query.clone();
        let client_context = client_context.clone();
        async move {
            log::info!("Parsing query: {:?}", query);
            if let Ok(address) = Pubkey::from_str(&query) {
                // Fetch the account
                match client_context
                    .read()
                    .get_account(address)
                    .await
                    .unwrap_or(None)
                {
                    Some(account) => {
                        // If account belongs to the thread program, go to /programs/thread/:address
                        if account.owner.eq(&clockwork_thread_program_v1::ID) {
                            let d = &account.data[..8];
                            if d.eq(&clockwork_thread_program_v1::state::Thread::discriminator()) {
                                return vec![SearchResult {
                                    title: format!("Go to thread {}", address.abbreviated()),
                                    route: format!("/programs/threads/{}", address),
                                }];
                            }
                        }

                        // If account belongs to the thread program, go to /programs/thread/:address
                        if account.owner.eq(&clockwork_thread_program_v2::ID) {
                            let d = &account.data[..8];
                            if d.eq(&clockwork_thread_program_v2::state::Thread::discriminator()) {
                                return vec![SearchResult {
                                    title: format!("Go to thread {}", address.abbreviated()),
                                    route: format!("/programs/threads/{}", address),
                                }];
                            }
                        }

                        vec![SearchResult {
                            title: format!("Go to account {}", address.abbreviated()),
                            route: format!("/accounts/{}", address),
                        }]
                    }
                    None => {
                        vec![SearchResult {
                            title: format!("Go to account {}", address.abbreviated()),
                            route: format!("/accounts/{}", address),
                        }]
                    }
                }
            } else {
                // TODO Display "invalid address" error to user
                log::info!("Invalid address");
                vec![]
            }
        }
    });

    if let Some(search_results) = results.value() {
        cx.render(rsx! {
            div {
                class: "flex flex-col w-full",
                for search_result in search_results.iter() {
                    rsx! {
                        SearchResultRow {
                            result: search_result.clone(),
                        }
                    }
                }
            }
        })
    } else {
        None
    }
}

#[derive(PartialEq, Props)]
pub struct SearchResultRowProps {
    result: SearchResult,
}

pub fn SearchResultRow(cx: Scope<SearchResultRowProps>) -> Element {
    let route = &cx.props.result.route;
    let title = &cx.props.result.title;
    let search_state = use_shared_state::<SearchState>(cx).unwrap();
    cx.render(rsx! {
        Link {
            to: route,
            class: "flex flex-row gap-x-2 mx-2 p-3 text-slate-100 transition hover:bg-slate-800 active:bg-slate-100 active:text-slate-900 rounded last:mb-2",
            onclick: move |_| {
                let mut w_search_state = search_state.write();
                w_search_state.active = false;
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
                    d: "M13.5 4.5L21 12m0 0l-7.5 7.5M21 12H3",
                }
            }
            p {
                class: "text-base my-auto",
                "{title}"
            }
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
                w_search_state.active = true;
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
