use dioxus::prelude::*;
use dioxus_router::use_router;
use gloo_events::EventListener;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlElement;

use crate::SearchState;

pub fn HotKeys(cx: Scope) -> Element {
    let router = use_router(&cx);
    let search_state = use_shared_state::<SearchState>(cx).unwrap();
    use_future(&cx, (), |_| {
        let router = router.clone();
        let search_state = search_state.clone();
        async move {
            let document = gloo_utils::document();
            let mut goto_mode = false;
            let mut list_index: Option<usize> = None;
            Some(EventListener::new(&document, "keydown", move |event| {
                let document = gloo_utils::document();
                let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                match event.key().as_str() {
                    // "Esc" | "Escape" => {
                    //     log::info!("Escape!");
                    //     let mut w_search_state = search_state.write();
                    //     w_search_state.active = false;
                    //     w_search_state.query = "".to_string();
                    // }
                    "/" => {
                        let mut w_search_state = search_state.write();
                        w_search_state.active = !w_search_state.active;
                        w_search_state.query = "".to_string();
                    }
                    "G" | "g" => {
                        goto_mode = true;
                    }
                    "A" | "a" => {
                        if goto_mode {
                            router.navigate_to("/accounts");
                            goto_mode = false;
                        }
                    }
                    "F" | "f" => {
                        if goto_mode {
                            router.navigate_to("/files");
                            goto_mode = false;
                        }
                    }
                    "H" | "h" => {
                        if goto_mode {
                            router.navigate_to("/");
                            goto_mode = false;
                        }
                    }
                    "P" | "p" => {
                        if goto_mode {
                            router.navigate_to("/programs");
                            goto_mode = false;
                        }
                    }
                    "K" | "k" => {
                        if goto_mode {
                            router.navigate_to("/keys");
                            goto_mode = false;
                        }
                    }
                    "J" | "j" => {
                        goto_mode = false;
                        let id = list_index.map_or(0, |i| i + 1);
                        let elem_id = format!("list-item-{}", id);
                        if let Some(element) = document.get_element_by_id(&*elem_id) {
                            if element.unchecked_into::<HtmlElement>().focus().is_ok() {
                                list_index = Some(id);
                            }
                        }
                    }
                    "K" | "k" => {
                        goto_mode = false;
                        let id = list_index.map_or(0, |i| i.saturating_sub(1));
                        let elem_id = format!("list-item-{}", id);
                        if let Some(element) = document.get_element_by_id(&*elem_id) {
                            if element.unchecked_into::<HtmlElement>().focus().is_ok() {
                                list_index = Some(id);
                            }
                        }
                    }
                    _ => {}
                }
            }))
        }
    });
    cx.render(rsx! {
        div {}
    })
}
