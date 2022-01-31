import { airdrop } from "./airdrop";
import {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmRawTransaction,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";

export async function newSigner(connection: Connection): Promise<Keypair> {
  const signer = Keypair.generate();
  await airdrop(2, signer.publicKey, connection);
  return signer;
}

export async function signAndSubmit(
  connection: Connection,
  ixs: Array<TransactionInstruction>,
  signer: Keypair
) {
  let tx = await newTx(connection, signer.publicKey);
  tx.add(...ixs);
  tx.partialSign(signer);
  await sendAndConfirmRawTransaction(connection, tx.serialize(), {});
}

export async function newTx(
  connection: Connection,
  feePayer: PublicKey
): Promise<Transaction> {
  const tx = new Transaction();
  tx.feePayer = feePayer;
  tx.recentBlockhash = (await connection.getRecentBlockhash()).blockhash;
  return tx;
}
