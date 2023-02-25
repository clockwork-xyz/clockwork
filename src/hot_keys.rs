use dioxus::prelude::*;
use dioxus_router::use_router;
use gloo_events::EventListener;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlElement;

pub fn HotKeys(cx: Scope) -> Element {
    let router = use_router(&cx);
    use_future(&cx, (), |_| {
        let router = router.clone();
        async move {
            let document = gloo_utils::document();
            let mut goto_mode = false;
            let mut list_index: Option<usize> = None;
            Some(EventListener::new(&document, "keydown", move |event| {
                let document = gloo_utils::document();
                let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                if goto_mode {
                    goto_mode = false;
                    match event.key().as_str() {
                        "D" | "d" => {
                            router.navigate_to("/data");
                        }
                        "F" | "f" => {
                            router.navigate_to("/files");
                        }
                        "H" | "h" => {
                            router.navigate_to("/");
                        }
                        "P" | "p" => {
                            router.navigate_to("/programs");
                        }
                        _ => {}
                    }
                } else {
                    match event.key().as_str() {
                        "G" | "g" => {
                            goto_mode = true;
                        }
                        "J" | "j" => {
                            let id = list_index.map_or(0, |i| i + 1);
                            let elem_id = format!("list-item-{}", id);
                            if let Some(element) = document.get_element_by_id(&*elem_id) {
                                if element.unchecked_into::<HtmlElement>().focus().is_ok() {
                                    list_index = Some(id);
                                }
                            }
                        }
                        "K" | "k" => {
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
                }
            }))
        }
    });
    cx.render(rsx! {
        div {}
    })
}
