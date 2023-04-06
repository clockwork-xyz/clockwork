use {
    anchor_lang::prelude::Pubkey,
    serde::{
        Deserialize,
        Serialize,
    },
    solana_client_wasm::solana_sdk::account::Account,
};

#[derive(Default, Serialize, Deserialize)]
pub struct User {
    pub pubkey: Option<Pubkey>,
    pub account: Option<Account>,
}
