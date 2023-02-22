use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use dioxus::prelude::*;
use pyth_sdk_solana::state::PriceAccount;
use solana_client_wasm::WasmClient;

use super::Page;

static DATA_FEED_PUBKEY: &str = "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG";

enum PythFeed {
    AptUsd,
    AtomUsd,
    BonkUsd,
    BtcUsd,
    BusdUsd,
    DaiUsd,
    EthUsd,
    MaticUsd,
    SolUsd,
    UsdcUsd,
    UsdtUsd,
}

impl PythFeed {
    pub fn all_pubkeys() -> Vec<Pubkey> {
        vec![
            PythFeed::AptUsd.pubkey(),
            PythFeed::AtomUsd.pubkey(),
            PythFeed::BonkUsd.pubkey(),
            PythFeed::BtcUsd.pubkey(),
            PythFeed::BusdUsd.pubkey(),
            PythFeed::DaiUsd.pubkey(),
            PythFeed::EthUsd.pubkey(),
            PythFeed::MaticUsd.pubkey(),
            PythFeed::SolUsd.pubkey(),
            PythFeed::UsdcUsd.pubkey(),
            PythFeed::UsdtUsd.pubkey(),
        ]
    }
    pub fn all_names() -> Vec<String> {
        vec![
            PythFeed::AptUsd.name().to_string(),
            PythFeed::AtomUsd.name().to_string(),
            PythFeed::BonkUsd.name().to_string(),
            PythFeed::BtcUsd.name().to_string(),
            PythFeed::BusdUsd.name().to_string(),
            PythFeed::DaiUsd.name().to_string(),
            PythFeed::EthUsd.name().to_string(),
            PythFeed::MaticUsd.name().to_string(),
            PythFeed::SolUsd.name().to_string(),
            PythFeed::UsdcUsd.name().to_string(),
            PythFeed::UsdtUsd.name().to_string(),
        ]
    }
    pub fn name(&self) -> &str {
        match self {
            PythFeed::AptUsd => "APT/USD",
            PythFeed::AtomUsd => "ATOM/USD",
            PythFeed::BonkUsd => "BONK/USD",
            PythFeed::BtcUsd => "BTC/USD",
            PythFeed::BusdUsd => "BUSD/USD",
            PythFeed::DaiUsd => "DAI/USD",
            PythFeed::EthUsd => "ETH/USD",
            PythFeed::SolUsd => "SOL/USD",
            PythFeed::MaticUsd => "MATIC/USD",
            PythFeed::UsdcUsd => "USDC/USD",
            PythFeed::UsdtUsd => "USDT/USD",
        }
    }
    pub fn pubkey(&self) -> Pubkey {
        let pubkey_str = match self {
            PythFeed::AptUsd => "FNNvb1AFDnDVPkocEri8mWbJ1952HQZtFLuwPiUjSJQ",
            PythFeed::AtomUsd => "CrCpTerNqtZvqLcKqz1k13oVeXV9WkMD2zA9hBKXrsbN",
            PythFeed::BonkUsd => "8ihFLu5FimgTQ1Unh4dVyEHUGodJ5gJQCrQf4KUVB9bN",
            PythFeed::BtcUsd => "GVXRSBjFk6e6J3NbVPXohDJetcTjaeeuykUpbQF8UoMU",
            PythFeed::BusdUsd => "7BHyT7XPMSA6LHYTgDTaeTPe3KTkKibMXZNxF5kiVsw1",
            PythFeed::DaiUsd => "CtJ8EkqLmeYyGB8s4jevpeNsvmD4dxVR2krfsDLcvV8Y",
            PythFeed::EthUsd => "JBu1AL4obBcCMqKBBxhpWCNUt136ijcuMZLFvTP7iWdB",
            PythFeed::SolUsd => "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG",
            PythFeed::MaticUsd => "7KVswB9vkCgeM3SHP7aGDijvdRAHK8P5wi9JXViCrtYh",
            PythFeed::UsdcUsd => "Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD",
            PythFeed::UsdtUsd => "3vxLXJqLqF3JG5TCbYycbKWRBbCJQLxQmBGCkyqEEefL",
        };
        Pubkey::from_str(pubkey_str).unwrap()
    }
}

#[derive(Clone, PartialEq)]
pub struct NamedPrice {
    name: String,
    price: PriceAccount,
}

pub fn DataPage(cx: Scope) -> Element {
    let pyth_feeds = use_state(&cx, || vec![]);

    use_future(&cx, (), |_| {
        let client = WasmClient::new("http://74.118.139.244:8899");
        let pyth_feeds = pyth_feeds.clone();
        let pyth_feed_pubkeys = PythFeed::all_pubkeys();
        let pyth_feed_names = PythFeed::all_names();
        async move {
            loop {
                let accounts = client
                    .get_multiple_accounts(&pyth_feed_pubkeys)
                    .await
                    .unwrap();
                let pyth_accounts = accounts
                    .iter()
                    .enumerate()
                    .filter_map(|(i, account)| {
                        if let Some(acc) = account {
                            Some(NamedPrice {
                                name: pyth_feed_names.get(i).unwrap().clone(),
                                price: *load::<PriceAccount>(acc.data.as_slice()).unwrap(),
                            })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<NamedPrice>>();
                pyth_feeds.set(pyth_accounts);
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
            for feed in pyth_feeds.get() {
                PriceRow {
                    price: feed.clone(),
                }
            }
        }
    })
}

pub fn PriceTableHeader(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "w-full flex flex-row justify-between",
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
    price: NamedPrice,
}

pub fn PriceRow(cx: Scope<PriceRowProps>) -> Element {
    let quote = cx.props.price.price.quote();
    cx.render(rsx! {
        div {
            class: "w-full flex flex-row justify-between",
            p {
                "{cx.props.price.name}"
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
