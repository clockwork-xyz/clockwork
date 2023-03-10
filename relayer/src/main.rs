use std::{fs, path::Path, str::FromStr};

use actix_cors::Cors;
use actix_web::{get, post, web, App, HttpServer, Responder};
use anchor_lang::{prelude::Pubkey, AccountDeserialize};
use clockwork_relayer_api::{
    Relay, SecretApprove, SecretCreate, SecretGet, SecretList, SecretListResponse, SecretRevoke,
    SignedRequest,
};
use clockwork_webhook_program::state::{HttpMethod, Webhook};
use rayon::prelude::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_zk_token_sdk::encryption::elgamal::{ElGamalCiphertext, ElGamalKeypair};

static ENCRYPTION_KEYPAIR_PATH: &str = "/home/ubuntu/encryption-keypair.json";
// static RELAYER_KEYPAIR_PATH: &str = "/home/ubuntu/relayer-keypair.json";
static SECRETS_PATH: &str = "/home/ubuntu/secrets";
static RPC_URL: &str = "http://127.0.0.1:8899";
// static RPC_URL: &str = "http://74.118.139.244:8899";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Generate a keypair for encryption.
    let encryption_keypair_path = Path::new(ENCRYPTION_KEYPAIR_PATH.into());
    if !encryption_keypair_path.exists() {
        let encryption_keypair = ElGamalKeypair::new_rand();
        encryption_keypair
            .write_json_file(ENCRYPTION_KEYPAIR_PATH)
            .expect("Failed to write encryption keypair to filepath");
    }

    // Verify the secrets directory exists.
    let secrets_path = Path::new(SECRETS_PATH.into());
    assert!(secrets_path.is_dir());

    // Start the webserver.
    HttpServer::new(|| {
        let cors = Cors::permissive()
            // ::default()
            // .allow_any_origin()
            // .disable_preflight()
            // .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            // .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            // .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .wrap(cors)
            .service(health)
            .service(relay)
            .service(secret_create)
            .service(secret_get)
            .service(secret_list)
            .service(secret_approve)
            .service(secret_revoke)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

#[get("/health")]
async fn health() -> impl Responder {
    "Ok"
}

#[post("/relay")]
async fn relay(req: web::Json<Relay>) -> impl Responder {
    let client = RpcClient::new_with_commitment(RPC_URL.into(), CommitmentConfig::processed());
    let data = client.get_account_data(&req.webhook).await.unwrap();
    let webhook = Webhook::try_deserialize(&mut data.as_slice()).unwrap();

    // TODO Check if this webhook has already been processed.
    // TODO Acquire a write lock on the webhook pubkey.

    // Begin building the request.
    let client = reqwest::Client::new();
    let url = hydrate_secret(webhook.url, webhook.authority).await;
    let mut request = match webhook.method {
        HttpMethod::Get => client.get(url),
        HttpMethod::Post => client.post(url),
    };

    // Add the request headers.
    for (k, v) in webhook.headers {
        request = request
            .try_clone()
            .unwrap()
            .header(k, hydrate_secret(v, webhook.authority).await);
    }

    // TODO Write the result back on-chain.
    match dbg!(request).send().await {
        Ok(response) => {
            // TODO
            dbg!(response);
        }
        Err(err) => {
            // TODO
            dbg!(err);
        }
    }

    "Ok"
}

#[post("/secret_create")]
async fn secret_create(req: web::Json<SignedRequest<SecretCreate>>) -> impl Responder {
    // Authenticate the request.
    assert!(req.0.authenticate());

    // Encrypt the secret word.
    let keypair = &ElGamalKeypair::read_json_file(ENCRYPTION_KEYPAIR_PATH).unwrap();
    let plaintext = req.msg.word.to_string();
    let ciphertext = encrypt(keypair, plaintext);

    // Save the ciphertext to the filesystem.
    let secrets_path = Path::new(SECRETS_PATH.into());
    assert!(secrets_path.is_dir());
    let user_secrets_path = secrets_path.join(req.signer.to_string());
    if !user_secrets_path.exists() {
        fs::create_dir(user_secrets_path.clone()).unwrap();
    }
    let secret_filepath = user_secrets_path.join(format!("{}.txt", req.msg.name));
    let filetext = serde_json::to_vec(&Secret {
        created_at: 123,
        delegates: vec![],
        ciphertext,
    })
    .unwrap();
    fs::write(secret_filepath, filetext).unwrap();
    "Ok"
}

#[derive(Deserialize, Serialize)]
pub struct Secret {
    pub created_at: u64,
    pub delegates: Vec<Pubkey>,
    pub ciphertext: Vec<u8>,
}

#[post("/secret_get")]
async fn secret_get(req: web::Json<SignedRequest<SecretGet>>) -> impl Responder {
    // Authenticate the request.
    assert!(req.0.authenticate());

    // Decrypt the ciphertext.
    // fetch_decrypted_secret(req.signer, req.msg.name.to_string())
    //     .await
    //     .unwrap_or("Not found".into())
    fetch_secret(req.signer, req.msg.name.to_string())
        .await
        .map_or("Not found".into(), |s| serde_json::to_string(&s).unwrap())
    // .unwrap_or("Not found".into())
}

#[post("/secret_list")]
async fn secret_list(req: web::Json<SignedRequest<SecretList>>) -> impl Responder {
    // Authenticate the request.
    assert!(req.0.authenticate());

    // Read the filepaths from the user's secrets directory.
    let secrets_path = Path::new(SECRETS_PATH.into());
    assert!(secrets_path.is_dir());
    let user_secrets_path = secrets_path.join(req.signer.to_string());
    if user_secrets_path.exists() && user_secrets_path.is_dir() {
        let paths = user_secrets_path.read_dir().unwrap();
        web::Json(SecretListResponse {
            secrets: paths
                .map(|path| path.unwrap().file_name().into_string().unwrap())
                .collect::<Vec<String>>(),
        })
    } else {
        web::Json(SecretListResponse { secrets: vec![] })
    }
}

#[post("/secret_approve")]
async fn secret_approve(req: web::Json<SignedRequest<SecretApprove>>) -> impl Responder {
    // Authenticate the request.
    assert!(req.0.authenticate());

    // Create and validate filepaths.
    let secrets_path = Path::new(SECRETS_PATH.into());
    assert!(secrets_path.is_dir());
    let user_secrets_path = secrets_path.join(req.signer.to_string());
    let secret_path = user_secrets_path.join(format!("{}.txt", req.msg.name));
    let secret_delegates_path = user_secrets_path.join(format!("{}.delegates", req.msg.name));
    assert!(secret_path.exists());

    // Read the list of current delegates.
    let mut delegates = if secret_delegates_path.exists() {
        fs::read_to_string(secret_delegates_path.clone()).unwrap()
    } else {
        "".to_string()
    };

    // Add delegate to the delegates file.
    if !delegates.contains(req.msg.delegate.to_string().as_str()) {
        delegates.push_str(format!("{}\n", req.msg.delegate).as_str());
    }
    fs::write(secret_delegates_path, delegates).unwrap();

    "Ok"
}

#[post("/secret_revoke")]
async fn secret_revoke(req: web::Json<SignedRequest<SecretRevoke>>) -> impl Responder {
    // Authenticate the request.
    assert!(req.0.authenticate());

    // Create and validate filepaths.
    let secrets_path = Path::new(SECRETS_PATH.into());
    assert!(secrets_path.is_dir());
    let user_secrets_path = secrets_path.join(req.signer.to_string());
    let secret_path = user_secrets_path.join(format!("{}.txt", req.msg.name));
    let secret_delegates_path = user_secrets_path.join(format!("{}.delegates", req.msg.name));
    assert!(secret_path.exists());

    // Read the list of current delegates.
    if secret_delegates_path.exists() {
        let mut delegates = fs::read_to_string(secret_delegates_path.clone()).unwrap();
        delegates = delegates.replace(format!("{}\n", req.msg.delegate).as_str(), "");
        fs::write(secret_delegates_path, delegates).unwrap();
    }

    "Ok"
}

const NORMALIZED_SECRET_LENGTH: usize = 128;
const PLAINTEXT_CHUNK_SIZE: usize = 4;
const CIPHERTEXT_CHUNK_SIZE: usize = 64;

fn decrypt(keypair: &ElGamalKeypair, ciphertext: Vec<u8>) -> String {
    // Decrypt the ciphertext chunks.
    let plaintext_bytes: Vec<u8> = ciphertext
        .par_chunks(CIPHERTEXT_CHUNK_SIZE)
        .map(|i| {
            let cx = ElGamalCiphertext::from_bytes(&i).unwrap();
            let dx = keypair.secret.decrypt_u32(&cx).unwrap();
            dx.to_le_bytes()[0..PLAINTEXT_CHUNK_SIZE].to_vec()
        })
        .flatten()
        .collect();

    // Lookup the plaintext length and take the slice from deciphered text.
    // Map the resulting bytes back into a utf8 string.
    let len = plaintext_bytes[NORMALIZED_SECRET_LENGTH - 1] as usize;
    let plaintext = plaintext_bytes.get(0..len).unwrap().to_vec();
    String::from_utf8(plaintext).unwrap()
}

fn encrypt(keypair: &ElGamalKeypair, plaintext: String) -> Vec<u8> {
    // Use the first byte to store the length.
    let bytes = &mut plaintext.as_bytes().to_vec();
    let len = bytes.len();
    assert!(len < NORMALIZED_SECRET_LENGTH);
    bytes.resize(NORMALIZED_SECRET_LENGTH - 1, 0);
    bytes.push(len as u8);

    // Chunk plaintext bytes into pieces of 32 bits each.
    // Encrypt each chunk into a 64 byte ciphertext.
    // Flatten the ciphertext bytes into a buffer.
    bytes
        .chunks(PLAINTEXT_CHUNK_SIZE)
        .map(|i| i.try_into().unwrap())
        .map(|s: [u8; PLAINTEXT_CHUNK_SIZE]| {
            keypair
                .public
                .encrypt(unsafe { std::mem::transmute::<[u8; PLAINTEXT_CHUNK_SIZE], u32>(s) })
                .to_bytes()
                .to_vec()
        })
        .flatten()
        .collect()
}

async fn fetch_decrypted_secret(user: Pubkey, name: String) -> Option<String> {
    let keypair = &ElGamalKeypair::read_json_file(ENCRYPTION_KEYPAIR_PATH).unwrap();
    let secret_filepath = Path::new(SECRETS_PATH.into())
        .join(user.to_string())
        .join(format!("{}.txt", name));
    if let Ok(filetext) = fs::read(secret_filepath) {
        let secret: Secret = serde_json::from_slice(&filetext).unwrap();
        let plaintext = decrypt(keypair, secret.ciphertext);
        Some(plaintext)
    } else {
        None
    }
}

async fn fetch_secret(user: Pubkey, name: String) -> Option<Secret> {
    let secret_filepath = Path::new(SECRETS_PATH.into())
        .join(user.to_string())
        .join(format!("{}.txt", name));
    if let Ok(filetext) = fs::read(secret_filepath) {
        let secret: Secret = serde_json::from_slice(&filetext).unwrap();
        Some(secret)
    } else {
        None
    }
}

fn is_approved(delegate: Pubkey, user: Pubkey, secret_name: String) -> bool {
    // Read the list of current delegates.
    let secret_delegates_path = Path::new(SECRETS_PATH.into())
        .join(user.to_string())
        .join(format!("{}.delegates", secret_name));
    let delegates = if secret_delegates_path.exists() {
        fs::read_to_string(secret_delegates_path.clone()).unwrap()
    } else {
        "".to_string()
    };

    // Return true if the user owns the secret or is approved to use it.
    delegate.eq(&user) || delegates.contains(delegate.to_string().as_str())
}

async fn hydrate_secret(phrase: String, user: Pubkey) -> String {
    let re = Regex::new(r"\{[[:alnum:]]+:[[:alnum:]]+\}").unwrap();
    if let Some(m) = re.find(&phrase.clone()) {
        let mut hydrated_phrase = phrase.clone();
        if let Some(secret_id) = phrase.as_str().get(m.start() + 1..m.end() - 1) {
            let mut parts = secret_id.split(':');
            if parts.clone().count() == 2 {
                // Verify the webhook.authority has permission to use this secret
                let secret_owner = Pubkey::from_str(parts.next().unwrap()).unwrap();
                let secret_name = parts.next().unwrap();
                if is_approved(user, secret_owner, secret_name.to_string()) {
                    if let Some(secret_word) =
                        fetch_decrypted_secret(secret_owner, secret_name.into()).await
                    {
                        hydrated_phrase.replace_range(m.start()..m.end(), &secret_word);
                    }
                }
            }
        }
    }
    phrase
}

#[cfg(test)]
mod tests {
    use solana_zk_token_sdk::encryption::elgamal::ElGamalKeypair;

    use crate::{decrypt, encrypt};

    #[test]
    fn test_encrypt_decrypt_correctness() {
        let keypair = &ElGamalKeypair::new_rand();
        let plaintext = "Hello, world";
        let ciphertext = encrypt(keypair, plaintext.into());
        let decrypted_plaintext = decrypt(keypair, ciphertext);
        assert!(plaintext.eq(&decrypted_plaintext));
    }
}
