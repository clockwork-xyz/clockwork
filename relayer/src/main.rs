use std::{fs, path::Path};

use actix_web::{get, post, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use solana_sdk::signature::read_keypair_file;
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

#[derive(Deserialize, Serialize)]
struct SecretCreate {
    name: String,
    word: String,
}

#[post("/secret")]
async fn secret_create(req: web::Json<SecretCreate>) -> impl Responder {
    // Encrypt the secret word.
    let keypair = &ElGamalKeypair::read_json_file(ENCRYPTION_KEYPAIR_PATH).unwrap();
    let plaintext = req.word.to_string();
    let ciphertext = encrypt(keypair, plaintext);

    // Save the ciphertext to the filesystem.
    let secrets_path = Path::new(SECRETS_PATH.into());
    assert!(secrets_path.is_dir());
    let secret_filepath = dbg!(secrets_path.join(format!("{}.txt", req.name)));
    fs::write(secret_filepath, ciphertext).unwrap();

    // TODO Save the ciphertext to Shadow Drive.
    // let keypair = read_keypair_file(RELAYER_KEYPAIR_PATH).unwrap();
    // let shdw_drive_client = ShadowDriveClient::new(keypair, "https://ssc-dao.genesysgo.net");
    // let decrypted_plaintext = decrypt(keypair, ciphertext);
    // dbg!(decrypted_plaintext);

    "Ok"
}

#[get("/secret/{name}")]
async fn secret_get(name: web::Path<String>) -> impl Responder {
    // Decrypt the ciphertext.
    let keypair = &ElGamalKeypair::read_json_file(ENCRYPTION_KEYPAIR_PATH).unwrap();
    let secret_filepath = Path::new(SECRETS_PATH.into()).join(format!("{}.txt", name));
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
        .chunks(CIPHERTEXT_CHUNK_SIZE)
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
