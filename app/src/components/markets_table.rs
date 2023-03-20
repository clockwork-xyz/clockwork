use dioxus::prelude::*;
use dioxus_router::Link;

use crate::pyth::{get_price_feeds, PythFeedPrice, Quotable};

pub fn MarketsTable(cx: Scope) -> Element {
    let market_data = use_state(&cx, || vec![]);

    use_future(&cx, (), |_| {
        let market_data = market_data.clone();
        async move {
            loop {
                market_data.set(get_price_feeds().await);
                gloo_timers::future::TimeoutFuture::new(1000).await;
            }
        }
    });

    cx.render(rsx! {
        div {
            class: "max-w-lg",
            h1 {
                class: "text-2xl font-semibold mb-6",
                "Markets"
            }
            table {
                class: "w-full",
                Header {}
                for (i, feed) in market_data.get().iter().enumerate() {
                    Row {
                        elem_id: format!("list-item-{}", i),
                        price: feed.clone(),
                    }
                }
            }
        }
    })
}

fn Header(cx: Scope) -> Element {
    cx.render(rsx! {
        thead {
            tr {
                class: "text-left text-sm text-slate-500",
                th {
                    class: "py-3 px-3 font-medium",
                    scope: "col",
                    "Ticker"
                }
                th {
                    class: "py-3 px-3 font-medium",
                    scope: "col",
                    "Price"
                }
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
    let address = cx.props.price.pubkey;
    let quote = cx.props.price.price.quote();
    let cell_class = "table-cell whitespace-nowrap first:pl-3 first:rounded-tl first:rounded-bl last:rounded-tr last:rounded-br py-2";
    cx.render(rsx! {
        Link {
            // class: "px-3 text-base hover:bg-slate-100 hover:text-slate-900 hover:cursor-pointer focus:bg-slate-900",
            to: "/accounts/markets/{address}",
            class: "table-row font-mono text-sm transition hover:cursor-pointer hover:bg-slate-800 active:bg-slate-100 active:text-slate-900",
            id: cx.props.elem_id.as_str(),
            div {
                // class: "whitespace-nowrap px-3 py-4",
                class: cell_class,
                "{cx.props.price.ticker}"
            }
            div {
                // class: "whitespace-nowrap px-3 py-4",
                class: cell_class,
                "{quote}"
            }
        }
    })
}
