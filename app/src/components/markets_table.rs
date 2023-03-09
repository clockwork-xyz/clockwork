use dioxus::prelude::*;

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
                class: "text-2xl font-semibold pb-2",
                "Markets"
            }
            table {
                class: "min-w-full divide-y divide-gray-300",
                Header {}
                tbody {
                    for (i, feed) in market_data.get().iter().enumerate() {
                        Row {
                            elem_id: format!("list-item-{}", i),
                            price: feed.clone(),
                        }
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
                th {
                    class: "py-3.5 text-left text-sm font-semibold sm:pl-3",
                    scope: "col",
                    "Ticker"
                }
                th {
                    class: "py-3.5 text-left text-sm font-semibold sm:pl-3",
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
    let quote = cx.props.price.price.quote();
    cx.render(rsx! {
        tr {
            class: "px-3 text-base border-b border-slate-800 hover:bg-slate-100 hover:text-slate-900 hover:cursor-pointer focus:bg-slate-900",
            id: cx.props.elem_id.as_str(),
            td {
                class: "whitespace-nowrap px-3 py-4",
                "{cx.props.price.ticker}"
            }
            td {
                class: "whitespace-nowrap px-3 py-4",
                "{quote}"
            }
        }
    })
}
