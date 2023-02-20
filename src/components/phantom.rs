use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectResponse {
    pub public_key: String,
}

#[wasm_bindgen]
extern "C" {
    pub type Solana;
    pub static solana: Solana;
    #[wasm_bindgen(method, getter=isConnected)]
    pub fn is_connected(this: &Solana) -> bool;
    #[wasm_bindgen(method, getter=isPhantom)]
    pub fn is_phantom(this: &Solana) -> bool;
    #[wasm_bindgen(method)]
    pub async fn connect(this: &Solana) -> JsValue;
    #[wasm_bindgen(method)]
    pub async fn disconnect(this: &Solana);
}
