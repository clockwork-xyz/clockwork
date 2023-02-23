// use dioxus::events::*;
use dioxus::prelude::*;
// use dioxus_router::Link;
use gloo_events::EventListener;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlElement;

use crate::pyth::PythFeed;
use crate::pyth::{get_price_feeds, PythFeedPrice, Quotable};

pub fn PriceFeedsTable(cx: Scope) -> Element {
    let pyth_feeds = use_state(&cx, || vec![]);
    let current_index = use_ref::<Option<usize>>(&cx, || None);

    use_future(&cx, (), |_| {
        let pyth_feeds = pyth_feeds.clone();
        async move {
            loop {
                pyth_feeds.set(get_price_feeds().await);
                gloo_timers::future::TimeoutFuture::new(1000).await;
            }
        }
    });

    use_effect(&cx, (), |_| {
        let current_index = current_index.clone();
        async move {
            *current_index.write() = None;
        }
    });

    use_future(&cx, (), |_| {
        let current_index = current_index.clone();
        async move {
            let document = gloo_utils::document();
            let len = PythFeed::all_pubkeys().len();
            Some(EventListener::new(&document, "keydown", move |event| {
                let document = gloo_utils::document();
                let idx = current_index.read();
                let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                match event.key().as_str() {
                    "J" | "j" => {
                        let id = format!(
                            "price-feed-{}",
                            if let Some(i) = *idx {
                                (i + 1).min(len - 1)
                            } else {
                                0
                            }
                        );
                        log::info!("id: {}", id);
                        if let Some(element) = document.get_element_by_id(&*id) {
                            element.unchecked_into::<HtmlElement>().focus().ok();
                        }
                    }
                    "K" | "k" => {
                        let id = format!(
                            "price-feed-{}",
                            if let Some(i) = *idx {
                                i.saturating_sub(1)
                            } else {
                                0
                            }
                        );
                        log::info!("id: {}", id);
                        if let Some(element) = document.get_element_by_id(&*id) {
                            element.unchecked_into::<HtmlElement>().focus().ok();
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
                    current_index: current_index.clone(),
                    id: i, // format!("price-feed-{}", i),
                    elem_id: format!("price-feed-{}", i),
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
    id: usize,
    elem_id: String,
    price: PythFeedPrice<'a>,
    current_index: UseRef<Option<usize>>,
}

fn PriceRow<'a>(cx: Scope<'a, PriceRowProps<'a>>) -> Element {
    let quote = cx.props.price.price.quote();
    // let elem_id = format!("price-feed-{}", cx.props.id);
    // cx.props.id.to_string().as_str()
    // let elem_id = ["price-feed-", cx.props.id.to_string().as_str()].join("");
    cx.render(rsx! {
        a {
            href: "/data/pyth/{cx.props.price.pubkey}",
            class: "w-full flex flex-row justify-between py-3 border-b border-slate-800 hover:bg-slate-900 focus:bg-slate-900",
            id: cx.props.elem_id.as_str(),
            onfocus: move |_event| {
                let mut w_current_index = cx.props.current_index.write();
                *w_current_index = Some(cx.props.id);
            },
            p {
                "{cx.props.price.ticker}"
            }
            p {
                "{quote}"
            }
        }
        // Link {
        //     to: "/price_feed/{cx.props.price.pubkey}",
        //     id: format!("price-feed-{}", cx.props.id).as_str(), // cx.props.id.as_str(),
        //     // onfocus: move |event| {
        //     //     cx.props.current_index.write();
        //     // },
        //     // onfocus
        //     class: "w-full flex flex-row justify-between py-3 border-b border-slate-800 hover:bg-slate-900 focus:bg-slate-900",
        //     p {
        //         "{cx.props.price.ticker}"
        //     }
        //     p {
        //         "{quote}"
        //     }
        // }
    })
}
