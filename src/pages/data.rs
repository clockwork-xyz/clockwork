use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use dioxus::prelude::*;
use pyth_sdk_solana::state::PriceAccount;
use solana_client_wasm::WasmClient;

use super::Page;

static DATA_FEED_PUBKEY: &str = "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG";

pub fn DataPage(cx: Scope) -> Element {
    let price = use_state(&cx, || None);
    use_future(&cx, (), |_| {
        let price = price.clone();
        let client = WasmClient::new("http://74.118.139.244:8899");
        let feed_pubkey = Pubkey::from_str(DATA_FEED_PUBKEY).unwrap();
        async move {
            loop {
                let account_data = client.get_account_data(&feed_pubkey).await.unwrap();
                let pyth_price = load::<PriceAccount>(account_data.as_slice()).unwrap();
                price.set(Some(pyth_price.clone()));
                gloo_timers::future::TimeoutFuture::new(1000).await;
            }
        }
    });
    cx.render(rsx! {
        Page {
            h1 {
                class: "text-2xl font-semibold",
                "Data"
            }
            PriceTableHeader {}
            if price.is_some() {
                rsx! {
                    PriceRow {
                        price: price.unwrap()
                    }
                }
            } else {
                rsx! {
                    p {
                        class: "mx-auto",
                        "Loading..."
                    }
                }
            }
        }
    })
}

pub fn PriceTableHeader(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full flex flex-row justify-between font-medium",
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
pub struct PriceRowProps {
    price: PriceAccount,
}

pub fn PriceRow(cx: Scope<PriceRowProps>) -> Element {
    let quote = cx.props.price.quote();
    cx.render(rsx! {
        div {
            class: "w-full flex flex-row justify-between",
            p {
                "SOL/USDC"
            }
            p {
                "{quote}"
            }
        }
    })
}

trait Quotable {
    fn quote(&self) -> String;
}

impl Quotable for PriceAccount {
    fn quote(&self) -> String {
        let mut fprice = self.agg.price as f64;
        let mut fconf = self.agg.conf as f64;
        for _ in 0..self.expo.abs() {
            fprice = fprice / 10 as f64;
            fconf = fconf / 10 as f64;
        }
        format!("${:.3} ± {:.3}", fprice, fconf).into()
    }
}

use bytemuck::{cast_slice, from_bytes, try_cast_slice, Pod, PodCastError};
use std::mem::size_of;

fn load<T: Pod>(data: &[u8]) -> Result<&T, PodCastError> {
    let size = size_of::<T>();
    if data.len() >= size {
        Ok(from_bytes(cast_slice::<u8, u8>(try_cast_slice(
            &data[0..size],
        )?)))
    } else {
        Err(PodCastError::SizeMismatch)
    }
}
