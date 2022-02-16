use {
    solana_client_helpers::Client,
    solana_sdk::{instruction::Instruction, transaction::Transaction},
};

pub fn sign_and_submit(client: Client, ixs: &[Instruction], memo: &str) {
    println!("🤖 {}", memo);
    let mut tx = Transaction::new_with_payer(ixs, Some(&client.payer_pubkey()));
    tx.sign(&vec![&client.payer], client.latest_blockhash().unwrap());
    let sig = client.send_and_confirm_transaction(&tx).unwrap();
    println!("🔏 {:?}", sig);
}
