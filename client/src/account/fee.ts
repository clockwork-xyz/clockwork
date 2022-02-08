import { Gateway, findPDA } from "@cronos-so/utils";
import { PublicKey } from "@solana/web3.js";
import { Cronos } from "../idl";

const SEED_FEE = Buffer.from("fee");

export class FeeGateway extends Gateway<Cronos, Cronos["accounts"][3]> {
  public async pda(daemon: PublicKey) {
    return await findPDA([SEED_FEE, daemon.toBuffer()], this.program.programId);
  }
}
