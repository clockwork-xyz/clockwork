import { BN } from "@project-serum/anchor";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

const keypairs = { payer: "/Users/lesleychang/.config/solana/id.json" };

const config = {
  keypairPaths: keypairs,
  program: { network: "localnet", payer: keypairs.payer },
  transferTask: {
    amount: new BN(0.05 * LAMPORTS_PER_SOL),
  },
} as const;

export default config;
