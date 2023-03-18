use std::mem::size_of;
use std::str::FromStr;

use anchor_lang::prelude::Pubkey;
use bytemuck::{cast_slice, from_bytes, try_cast_slice, Pod, PodCastError};
use pyth_sdk_solana::state::PriceAccount;
use solana_client_wasm::WasmClient;

pub enum PythFeed {
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
    pub fn all_tickers<'a>() -> Vec<&'a str> {
        vec![
            PythFeed::AptUsd.ticker(),
            PythFeed::AtomUsd.ticker(),
            PythFeed::BonkUsd.ticker(),
            PythFeed::BtcUsd.ticker(),
            PythFeed::BusdUsd.ticker(),
            PythFeed::DaiUsd.ticker(),
            PythFeed::EthUsd.ticker(),
            PythFeed::MaticUsd.ticker(),
            PythFeed::SolUsd.ticker(),
            PythFeed::UsdcUsd.ticker(),
            PythFeed::UsdtUsd.ticker(),
        ]
    }
    pub fn ticker(&self) -> &str {
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
pub struct PythFeedPrice<'a> {
    pub price: PriceAccount,
    pub pubkey: Pubkey,
    pub ticker: &'a str,
}

pub async fn get_price_feeds<'a>() -> Vec<PythFeedPrice<'a>> {
    let client =
        WasmClient::new("https://rpc.helius.xyz/?api-key=cafb5acc-3dc2-47a0-8505-77ea5ebc7ec6");
    let pyth_feed_pubkeys = PythFeed::all_pubkeys();
    let pyth_feed_ticker = PythFeed::all_tickers();
    client
        .get_multiple_accounts(&pyth_feed_pubkeys)
        .await
        .unwrap()
        .iter()
        .enumerate()
        .filter_map(|(i, account)| {
            if let Some(acc) = account {
                Some(PythFeedPrice {
                    ticker: pyth_feed_ticker.get(i).unwrap(),
                    pubkey: *pyth_feed_pubkeys.get(i).unwrap(),
                    price: *load::<PriceAccount>(acc.data.as_slice()).unwrap(),
                })
            } else {
                None
            }
        })
        .collect::<Vec<PythFeedPrice>>()
}

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

pub trait Quotable {
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
        if fprice < 0.001 {
            format!("${:.3e} ± {:.3e}", fprice, fconf).into()
        } else {
            format!("${:.3} ± {:.3}", fprice, fconf).into()
        }
    }
}
