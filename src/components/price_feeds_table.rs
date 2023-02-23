use dioxus::prelude::*;
use dioxus_router::Link;
use gloo_events::EventListener;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlElement;

use crate::pyth::{get_price_feeds, PythFeedPrice, Quotable};

pub fn PriceFeedsTable(cx: Scope) -> Element {
    let pyth_feeds = use_state(&cx, || vec![]);
    let current_index = use_ref(&cx, || 0);

    use_future(&cx, (), |_| {
        let pyth_feeds = pyth_feeds.clone();
        async move {
            loop {
                pyth_feeds.set(get_price_feeds().await);
                gloo_timers::future::TimeoutFuture::new(1000).await;
            }
        }
    });

    use_future(&cx, (), |_| {
        let current_index = current_index.clone();
        async move {
            let document = gloo_utils::document();
            Some(EventListener::new(&document, "keydown", move |event| {
                let document = gloo_utils::document();
                let idx = current_index.read();
                let id = format!("price-feed-{}", idx);
                let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                match event.key().as_str() {
                    "J" | "j" => {
                        if let Some(element) = document.get_element_by_id(&*id) {
                            if element.unchecked_into::<HtmlElement>().focus().is_ok() {
                                // current_index = &Some(0);
                            }
                        }
                    }
                    "K" | "k" => {
                        if let Some(element) = document.get_element_by_id(&*id) {
                            element.unchecked_into::<HtmlElement>().focus().unwrap();
                        }
                    }
                    _ => {}
                }
            }))
        }
    });

    cx.render(rsx! {
        div {
            PriceTableHeader {}
            for (i, feed) in pyth_feeds.get().iter().enumerate() {
                PriceRow {
                    id: format!("price-feed-{}", i),
                    price: feed.clone(),
                }
            }
        }
    })
}

fn PriceTableHeader(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full flex flex-row justify-between py-3 border-b border-slate-800 font-medium text-sm text-slate-600",
            p {
                "Ticker"
            }
            p {
                "Price"
            }
        }
    })
}

#[derive(PartialEq, Props)]
struct PriceRowProps<'a> {
    id: String,
    price: PythFeedPrice<'a>,
}

fn PriceRow<'a>(cx: Scope<'a, PriceRowProps<'a>>) -> Element {
    let quote = cx.props.price.price.quote();
    cx.render(rsx! {
        Link {
            to: "/price_feed/{cx.props.price.pubkey}",
            id: cx.props.id.as_str(),
            class: "w-full flex flex-row justify-between py-3 border-b border-slate-800 hover:bg-slate-900 focus:bg-slate-900",
            p {
                "{cx.props.price.ticker}"
            }
            p {
                "{quote}"
            }
        }
    })
}
