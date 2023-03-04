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

fn decrypt(keypair: &ElGamalKeypair, ciphertext: Vec<u8>) -> String {
    // Decrypt the ciphertext chunks.
    let plaintext_bytes: Vec<u8> = ciphertext
        .chunks(64)
        .map(|i| {
            let cx = ElGamalCiphertext::from_bytes(&i).unwrap();
            let dx = keypair.secret.decrypt_u32(&cx).unwrap();
            dx.to_le_bytes()[0..4].to_vec()
        })
        .flatten()
        .collect();

    // Lookup the plaintext length and take the slice from deciphered text.
    // Map the resulting bytes back into a utf8 string.
    let len = plaintext_bytes[0] as usize;
    let plaintext = plaintext_bytes.get(1..len + 1).unwrap().to_vec();
    String::from_utf8(plaintext).unwrap()
}

fn encrypt(keypair: &ElGamalKeypair, plaintext: String) -> Vec<u8> {
    // Use the first byte to store the length.
    let plaintext_bytes = &mut plaintext.as_bytes();
    assert!(plaintext_bytes.len() < 256);
    let mut metadata_plaintext_bytes = vec![plaintext_bytes.len() as u8];
    metadata_plaintext_bytes.extend_from_slice(plaintext_bytes);

    // Chunk the [metadata + plaintext] buffer into 32 bits, and encrypt each chunk.
    // Flatten the resulting ciphertext into a buffer.
    metadata_plaintext_bytes
        .chunks(4)
        .map(|i| {
            if i.len() < 4 {
                // If the chunk is smaller than 4 bytes, pad with 0s.
                let mut x = i.to_vec();
                x.resize(4, 0);
                x.as_slice().try_into().unwrap()
            } else {
                i.try_into().unwrap()
            }
        })
        .map(|s: [u8; 4]| {
            keypair
                .public
                .encrypt(unsafe { std::mem::transmute::<[u8; 4], u32>(s) })
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
        let ciphertext = encrypt(keypair, plaintext.into());
        let decrypted_plaintext = decrypt(keypair, ciphertext);
        assert!(plaintext.eq(&decrypted_plaintext));
    }
}
