import { Gateway, findPDA } from "@chronos-so/utils";
import { PublicKey } from "@solana/web3.js";
import { ListProgram } from "../idl";

const SEED_LIST = Buffer.from("lst");

export class ListGateway extends Gateway<
  ListProgram,
  ListProgram["accounts"][1]
> {
  public async pda(owner: PublicKey, namespace: PublicKey) {
    return await findPDA(
      [SEED_LIST, owner.toBuffer(), namespace.toBuffer()],
      this.program.programId
    );
  }
}
