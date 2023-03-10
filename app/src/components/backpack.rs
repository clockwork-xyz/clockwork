#![allow(non_upper_case_globals)]

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Backpack;
    pub static backpack: Backpack;
    #[wasm_bindgen(method, getter=isConnected)]
    pub fn is_connected(this: &Backpack) -> bool;
    #[wasm_bindgen(method)]
    pub async fn connect(this: &Backpack) -> JsValue;
    #[wasm_bindgen(method)]
    pub async fn disconnect(this: &Backpack);
    #[wasm_bindgen(method, getter=publicKey)]
    pub fn pubkey(this: &Backpack) -> N;
    #[wasm_bindgen(method, js_name=signMessage)]
    pub async fn sign_message(this: &Backpack, message: Vec<u8>, pubkey: Option<N>) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    pub type N;
    #[wasm_bindgen(method, js_name = toString)]
    pub fn to_string(this: &N) -> String;
}
