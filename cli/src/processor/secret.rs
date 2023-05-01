use anchor_lang::prelude::Pubkey;
use clockwork_relayer_api::{
    SecretApprove, SecretCreate, SecretGet, SecretList, SecretRevoke, SignedRequest,
};
use reqwest::header::CONTENT_TYPE;
use solana_sdk::signer::Signer;

use crate::{client::Client, errors::CliError};

pub fn get(client: &Client, name: String) -> Result<(), CliError> {
    let keypair = &client.payer;
    let msg = SecretGet { name };
    let msg_bytes = bincode::serialize(&msg).unwrap();
    let sig = keypair.sign_message(&msg_bytes);
    let req = SignedRequest {
        msg,
        signer: keypair.pubkey(),
        signature: sig,
    };
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://0.0.0.0:8000/secret_get")
        .header(CONTENT_TYPE, "application/json")
        .json(&req)
        .send();
    if let Ok(plaintext) = res {
        println!("{:?}", plaintext.text());
    }
    Ok(())
}

pub fn list(client: &Client) -> Result<(), CliError> {
    let keypair = &client.payer;
    let msg = SecretList {};
    let msg_bytes = bincode::serialize(&msg).unwrap();
    let sig = keypair.sign_message(&msg_bytes);
    let req = SignedRequest {
        msg,
        signer: keypair.pubkey(),
        signature: sig,
    };
    let client = reqwest::blocking::Client::new();
    let res = client
        .post("http://0.0.0.0:8000/secret_list")
        .header(CONTENT_TYPE, "application/json")
        .json(&req)
        .send();
    if let Ok(plaintext) = res {
        println!("{:?}", plaintext.text());
    }
    Ok(())
}

pub fn create(client: &Client, name: String, word: String) -> Result<(), CliError> {
    let keypair = &client.payer;
    let msg = SecretCreate { name, word };
    let msg_bytes = bincode::serialize(&msg).unwrap();
    let sig = keypair.sign_message(&msg_bytes);
    let req = SignedRequest {
        msg,
        signer: keypair.pubkey(),
        signature: sig,
    };
    let client = reqwest::blocking::Client::new();
    client
        .post("http://0.0.0.0:8000/secret_create")
        .header(CONTENT_TYPE, "application/json")
        .json(&req)
        .send()
        .unwrap();
    Ok(())
}

pub fn approve(client: &Client, name: String, delegate: Pubkey) -> Result<(), CliError> {
    let keypair = &client.payer;
    let msg = SecretApprove { name, delegate };
    let msg_bytes = bincode::serialize(&msg).unwrap();
    let sig = keypair.sign_message(&msg_bytes);
    let req = SignedRequest {
        msg,
        signer: keypair.pubkey(),
        signature: sig,
    };
    let client = reqwest::blocking::Client::new();
    client
        .post("http://0.0.0.0:8000/secret_approve")
        .header(CONTENT_TYPE, "application/json")
        .json(&req)
        .send()
        .unwrap();
    Ok(())
}

pub fn revoke(client: &Client, name: String, delegate: Pubkey) -> Result<(), CliError> {
    let keypair = &client.payer;
    let msg = SecretRevoke { name, delegate };
    let msg_bytes = bincode::serialize(&msg).unwrap();
    let sig = keypair.sign_message(&msg_bytes);
    let req = SignedRequest {
        msg,
        signer: keypair.pubkey(),
        signature: sig,
    };
    let client = reqwest::blocking::Client::new();
    client
        .post("http://0.0.0.0:8000/secret_revoke")
        .header(CONTENT_TYPE, "application/json")
        .json(&req)
        .send()
        .unwrap();
    Ok(())
}
