import { Gateway, findPDA } from "@chronos-so/utils";
import { BN } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { Indexer } from "../idl";

const SEED_ELEMENT = Buffer.from("elm");

export class ElementGateway extends Gateway<Indexer, Indexer["accounts"][0]> {
  public async pda(index: PublicKey, position: BN) {
    return await findPDA(
      [SEED_ELEMENT, index.toBuffer(), position.toArrayLike(Buffer, "be", 16)],
      this.program.programId
    );
  }
}
