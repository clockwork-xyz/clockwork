use anchor_lang::prelude::Pubkey;
use solana_client_wasm::solana_sdk::account::Account;
pub struct User {
    pub pubkey: Option<Pubkey>,
    pub account: Option<Account>,
}
