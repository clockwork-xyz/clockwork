use anchor_lang::prelude::Pubkey;
use js_sys::WebAssembly::RuntimeError;
use serde::{Deserialize, Serialize};
use solana_client_wasm::solana_sdk::account::Account;
use std::str::FromStr;

#[derive(Default, Serialize, Deserialize)]
pub struct User {
    pub pubkey: Option<Pubkey>,
    pub account: Option<Account>,
    pub cluster: Cluster,
}

#[derive(Default, Serialize, Deserialize, Clone)]
pub enum Cluster {
    #[default]
    Mainnet,
    Devnet,
    Testnet,
}

impl Cluster {
    fn url(&self) -> String {
        match self {
            Self::Mainnet => "https://api.mainnet-beta.solana.com".to_string(),
            Self::Devnet => "https://api.devnet.solana.com".to_string(),
            Self::Testnet => "https://api.testnet.solana.com".to_string(),
        }
    }
}

impl ToString for Cluster {
    fn to_string(&self) -> String {
        match self {
            Self::Mainnet => "Mainnet".to_string(),
            Self::Devnet => "Devnet".to_string(),
            Self::Testnet => "Testnet".to_string(),
        }
    }
}

impl FromStr for Cluster {
    type Err = RuntimeError;

    fn from_str(expression: &str) -> Result<Self, Self::Err>  {
        match expression {
            "Mainnet" => Ok(Self::Mainnet),
            "Devnet" => Ok(Self::Devnet),
            "Testnet" => Ok(Self::Testnet),
            _ => Err(RuntimeError::new("Invalid expression")),
        }
    }
}
