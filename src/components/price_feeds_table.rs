use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pyth::{get_price_feeds, PythFeedPrice, Quotable};

pub fn PriceFeedsTable(cx: Scope) -> Element {
    let pyth_feeds = use_state(&cx, || vec![]);

    use_future(&cx, (), |_| {
        let pyth_feeds = pyth_feeds.clone();
        async move {
            loop {
                pyth_feeds.set(get_price_feeds().await);
                gloo_timers::future::TimeoutFuture::new(1000).await;
            }
        }
    });

    cx.render(rsx! {
        div {
            PriceTableHeader {}
            for feed in pyth_feeds.get() {
                PriceRow {
                    price: feed.clone(),
                }
            }
        }
    })
}

fn PriceTableHeader(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full flex flex-row justify-between py-3 border-b border-slate-800",
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
    price: PythFeedPrice<'a>,
}

fn PriceRow<'a>(cx: Scope<'a, PriceRowProps<'a>>) -> Element {
    let quote = cx.props.price.price.quote();
    cx.render(rsx! {
        Link {
            to: "/price_feed/{cx.props.price.pubkey}",
            class: "w-full flex flex-row justify-between py-3 border-b border-slate-800",
            p {
                "{cx.props.price.ticker}"
            }
            p {
                "{quote}"
            }
        }
    })
}
