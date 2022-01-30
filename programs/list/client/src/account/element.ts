import { Gateway, findPDA } from "@faktorfi/utils";
import { BN } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { ListProgram } from "../idl";

const SEED_ELEMENT = Buffer.from("elm");

export class ElementGateway extends Gateway<
  ListProgram,
  ListProgram["accounts"][0]
> {
  public async pda(index: PublicKey, position: BN) {
    return await findPDA(
      [SEED_ELEMENT, index.toBuffer(), position.toArrayLike(Buffer, "be", 16)],
      this.program.programId
    );
  }
}
