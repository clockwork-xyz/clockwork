use solana_sdk::signature::{read_keypair_file, Keypair};

pub fn read_or_new_keypair(keypath: Option<String>) -> Keypair {
    match keypath {
        Some(keypath) => read_keypair_file(keypath).unwrap(),
        None => Keypair::new(),
    }
}
