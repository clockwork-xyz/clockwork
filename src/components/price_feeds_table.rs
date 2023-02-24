use dioxus::prelude::*;

use crate::pyth::{get_price_feeds, PythFeedPrice, Quotable};

pub fn MarketsTable(cx: Scope) -> Element {
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
            Header {}
            for (i, feed) in pyth_feeds.get().iter().enumerate() {
                Row {
                    elem_id: format!("list-item-{}", i),
                    price: feed.clone(),
                }
            }
        }
    })
}

fn Header(cx: Scope) -> Element {
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
struct RowProps<'a> {
    elem_id: String,
    price: PythFeedPrice<'a>,
}

fn Row<'a>(cx: Scope<'a, RowProps<'a>>) -> Element {
    let quote = cx.props.price.price.quote();
    cx.render(rsx! {
        a {
            href: "/data/pyth/{cx.props.price.pubkey}",
            class: "w-full flex flex-row justify-between py-3 border-b border-slate-800 hover:bg-slate-900 focus:bg-slate-900",
            id: cx.props.elem_id.as_str(),
            p {
                "{cx.props.price.ticker}"
            }
            p {
                "{quote}"
            }
        }
    })
}
