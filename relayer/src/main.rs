use std::{fs, path::Path};

use actix_web::{post, web, App, HttpServer, Responder};
use clockwork_relayer_api::{SecretCreate, SecretGet, SignedRequest};
use rayon::prelude::*;
use solana_zk_token_sdk::encryption::elgamal::{ElGamalCiphertext, ElGamalKeypair};

static ENCRYPTION_KEYPAIR_PATH: &str = "/home/ubuntu/encryption-keypair.json";
static RELAYER_KEYPAIR_PATH: &str = "/home/ubuntu/relayer-keypair.json";
static SECRETS_PATH: &str = "/home/ubuntu/secrets";
// static RPC_URL: &str = "http://0.0.0.0:8899";

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
    HttpServer::new(|| App::new().service(secret_create).service(secret_get))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
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
    fs::write(secret_filepath, ciphertext).unwrap();

    // TODO Save the ciphertext to Shadow Drive.
    // let keypair = read_keypair_file(RELAYER_KEYPAIR_PATH).unwrap();
    // let shdw_drive_client = ShadowDriveClient::new(keypair, "https://ssc-dao.genesysgo.net");
    // let decrypted_plaintext = decrypt(keypair, ciphertext);
    // dbg!(decrypted_plaintext);

    "Ok"
}

#[post("/secret_get")]
async fn secret_get(req: web::Json<SignedRequest<SecretGet>>) -> impl Responder {
    // Authenticate the request.
    assert!(req.0.authenticate());

    // Decrypt the ciphertext.
    let keypair = &ElGamalKeypair::read_json_file(ENCRYPTION_KEYPAIR_PATH).unwrap();
    let secret_filepath = Path::new(SECRETS_PATH.into())
        .join(req.signer.to_string())
        .join(format!("{}.txt", req.msg.name));
    let ciphertext = fs::read(secret_filepath).unwrap();
    let plaintext = decrypt(keypair, ciphertext);
    plaintext
}

const NORMALIZED_SECRET_LENGTH: usize = 64;
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

#[cfg(test)]
mod tests {
    use solana_zk_token_sdk::encryption::elgamal::ElGamalKeypair;

    use crate::{decrypt, encrypt};

    #[test]
    fn test_encrypt_decrypt_correctness() {
        let keypair = &ElGamalKeypair::new_rand();
        let plaintext = "Hello, world";
        let ciphertext = dbg!(encrypt(keypair, plaintext.into()));
        let decrypted_plaintext = dbg!(decrypt(keypair, ciphertext));
        assert!(plaintext.eq(&decrypted_plaintext));
    }
}
