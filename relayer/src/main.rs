use std::str::FromStr;

use actix_web::{get, post, web, App, HttpRequest, HttpServer, Responder};
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};

static ACK_AUTHORITY_KEYPATH: &str = "/Users/garfield/Developer/http-server/keypair.json";
// static RPC_URL: &str = "http://0.0.0.0:8899";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

#[post("/relay")]
async fn hello(req: HttpRequest) -> impl Responder {
    // Parse headers to request metadata

    println!("Req: {:?}", req);
    // let caller_id = Pubkey::from_str(get_caller_id(&req).unwrap()).unwrap();
    // let request_id = Pubkey::from_str(get_request_id(&req).unwrap()).unwrap();
    // let worker_id = Pubkey::from_str(get_worker_id(&req).unwrap()).unwrap();
    // println!(
    //     "caller_id: {} request_id: {} worker_id: {}",
    //     caller_id, request_id, worker_id
    // );

    // Build the client
    let keypair = read_keypair_file(ACK_AUTHORITY_KEYPATH.to_string()).unwrap();

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
