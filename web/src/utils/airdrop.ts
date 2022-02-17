import { Connection, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

export async function airdrop(
  amount: number,
  to: PublicKey,
  connection: Connection
) {
  await connection
    .requestAirdrop(to, amount * LAMPORTS_PER_SOL)
    .then((sig) => connection.confirmTransaction(sig, "confirmed"));
}
