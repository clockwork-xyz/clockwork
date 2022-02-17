import NodeWallet from "@project-serum/anchor/dist/cjs/nodewallet";
import { Wallet } from "@project-serum/anchor/dist/cjs/provider";
import { Keypair } from "@solana/web3.js";

export function localWallet(filepath: String): Wallet {
  const keypair = Keypair.fromSecretKey(
    Buffer.from(
      JSON.parse(
        require("fs").readFileSync(filepath, {
          encoding: "utf-8",
        })
      )
    )
  );
  return new NodeWallet(keypair);
}
