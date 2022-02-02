import { Gateway, findPDA } from "@chronos-so/utils";
import { PublicKey } from "@solana/web3.js";
import { Indexer } from "../idl";

const SEED_LIST = Buffer.from("list");

export class ListGateway extends Gateway<Indexer, Indexer["accounts"][1]> {
  public async pda(owner: PublicKey, namespace: PublicKey) {
    return await findPDA(
      [SEED_LIST, owner.toBuffer(), namespace.toBuffer()],
      this.program.programId
    );
  }
}
