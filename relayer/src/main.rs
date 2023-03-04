use std::str::FromStr;

use actix_web::{get, post, web, App, HttpRequest, HttpServer, Responder};
use curve25519_dalek::scalar::Scalar;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
use solana_zk_token_sdk::encryption::elgamal::{ElGamalCiphertext, ElGamalKeypair};

// static ACK_AUTHORITY_KEYPATH: &str = "TODO";
// static RPC_URL: &str = "http://0.0.0.0:8899";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

#[post("/relay/{word}")]
async fn hello(req: HttpRequest, name: web::Path<String>) -> impl Responder {
    // Parse headers to request metadata

    println!("Req: {:?}", req);

    let keypair = &ElGamalKeypair::new_rand();
    let plaintext = name.to_string();
    let ciphertext = encrypt(keypair, plaintext);
    let decrypted_plaintext = decrypt(keypair, ciphertext);
    dbg!(decrypted_plaintext);

    // let ciphertext = keypair.public.encrypt(plaintext);

    // let caller_id = Pubkey::from_str(get_caller_id(&req).unwrap()).unwrap();
    // let request_id = Pubkey::from_str(get_request_id(&req).unwrap()).unwrap();
    // let worker_id = Pubkey::from_str(get_worker_id(&req).unwrap()).unwrap();
    // println!(
    //     "caller_id: {} request_id: {} worker_id: {}",
    //     caller_id, request_id, worker_id
    // );

    // Build the client
    // let keypair = read_keypair_file(ACK_AUTHORITY_KEYPATH.to_string()).unwrap();

    // let client = cronos_client::Client::new(keypair, RPC_URL.to_string());

    // Execute the ack instruction
    // web::block(move || {
    //     let ix = cronos_client::http::instruction::request_ack(
    //         client.payer_pubkey(),
    //         caller_id,
    //         request_id,
    //         worker_id,
    //     );
    //     let sig = client.send(&[ix], &[client.payer()]).unwrap();
    //     println!("Sig: {:#?}", sig);
    // })
    // .await
    // .ok();

    format!("Hello!")
}

// fn get_caller_id<'a>(req: &'a HttpRequest) -> Option<&'a str> {
//     req.headers().get("x-caller-id")?.to_str().ok()
// }

// fn get_request_id<'a>(req: &'a HttpRequest) -> Option<&'a str> {
//     req.headers().get("x-request-id")?.to_str().ok()
// }

// fn get_worker_id<'a>(req: &'a HttpRequest) -> Option<&'a str> {
//     req.headers().get("clockwork-worker")?.to_str().ok()
// }

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
