use serde::{Deserialize, Serialize};
use solana_sdk::{pubkey::Pubkey, signature::Signature};

#[derive(Deserialize, Serialize)]
pub struct SignedRequest<T: Sized> {
    pub msg: T,
    pub signer: Pubkey,
    pub signature: Signature,
}

impl<T: Serialize> SignedRequest<T> {
    pub fn authenticate(&self) -> bool {
        let msg_bytes = bincode::serialize(&self.msg).unwrap();
        self.signature
            .verify(&self.signer.to_bytes(), msg_bytes.as_slice())
    }
}

#[derive(Deserialize, Serialize)]
pub struct Relay {
    pub webhook: Pubkey,
}

#[derive(Deserialize, Serialize)]
pub struct SecretCreate {
    pub name: String,
    pub word: String,
}

#[derive(Deserialize, Serialize)]
pub struct SecretGet {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct SecretList {}

#[derive(Deserialize, Serialize)]
pub struct SecretListResponse {
    pub secrets: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct SecretApprove {
    pub name: String,
    pub delegate: Pubkey,
}

#[derive(Deserialize, Serialize)]
pub struct SecretRevoke {
    pub name: String,
    pub delegate: Pubkey,
}
