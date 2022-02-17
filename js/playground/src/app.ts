import { Token, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";

import config from "./config";

export default class App {
  constructor(private cronos = CronosClient.createClient(config.program)) {}

  async createTransferIx({ owner, daemonPubkey }: { owner: Keypair; daemonPubkey: PublicKey }) {
    const signerTokens = await Token.createWrappedNativeAccount(this.cronos.provider.connection, TOKEN_PROGRAM_ID, owner.publicKey, owner, 0);

    const daemonTokens = await Token.createWrappedNativeAccount(
      this.cronos.provider.connection,
      TOKEN_PROGRAM_ID,
      daemonPubkey,
      owner,
      LAMPORTS_PER_SOL * 0.3,
    );

    // Invoke a task
    const ix = Token.createTransferInstruction(TOKEN_PROGRAM_ID, daemonTokens, signerTokens, daemonPubkey, [], config.transferTask.amount.toNumber());

    return ix;
  }

  public async start(): Promise<void> {
    const ownerKeypair = toKeypair(config.keypairPaths.payer);

    const daemon = await this.cronos.getOrCreateDaemon(ownerKeypair);

    // // Invoke a task
    // const transferIx = await this.createTransferIx({ owner: ownerKeypair, daemonPubkey: daemon.owner });

    // await this.cronos.invoke({
    //   daemon: daemon.owner,
    //   ix: transferIx,
    //   owner: ownerKeypair,
    // });
  }
}
