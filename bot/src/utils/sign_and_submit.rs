use {
    solana_client_helpers::{Client, ClientResult},
    solana_sdk::{instruction::Instruction, signature::Signature, transaction::Transaction},
};

pub fn sign_and_submit(
    client: &Client,
    ixs: &[Instruction],
    memo: &str,
) -> ClientResult<Signature> {
    println!("{}", memo);
    let payer = client.payer_pubkey();
    let mut tx = Transaction::new_with_payer(ixs, Some(&payer));
    tx.sign(&vec![&client.payer], client.latest_blockhash()?);
    let sig = client.send_and_confirm_transaction(&tx)?;
    println!("✅ {:?}", sig);
    Ok(sig)
}
