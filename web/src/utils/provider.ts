import { Provider, setProvider } from "@project-serum/anchor";
import { Wallet } from "@project-serum/anchor/dist/cjs/provider";
import { ConfirmOptions, Connection } from "@solana/web3.js";

const opts: ConfirmOptions = {
  preflightCommitment: "processed",
};

export function loadProvider(endpoint: string, wallet: Wallet): Provider {
  const connection = new Connection(endpoint, opts);
  const provider = new Provider(connection, wallet, opts);
  setProvider(provider);
  return provider;
}
