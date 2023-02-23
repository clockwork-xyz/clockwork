use dioxus::prelude::*;
use dioxus_router::use_router;
use gloo_events::EventListener;
use wasm_bindgen::{JsCast, UnwrapThrowExt};

pub fn HotKeys(cx: Scope) -> Element {
    let router = use_router(&cx);
    use_future(&cx, (), |_| {
        let router = router.clone();
        async move {
            let document = gloo_utils::document();
            let mut goto_mode = false;
            Some(EventListener::new(&document, "keydown", move |event| {
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
                        "T" | "t" => {
                            router.navigate_to("/threads");
                        }
                        _ => {}
                    }
                } else {
                    match event.key().as_str() {
                        "G" | "g" => {
                            goto_mode = true;
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
