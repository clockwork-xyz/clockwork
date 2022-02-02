import { Gateway, findPDA } from "@chronos-so/utils";
import { PublicKey } from "@solana/web3.js";
import { Chronos } from "../idl";

const SEED_REVENUE = Buffer.from("revenue");

export class RevenueGateway extends Gateway<Chronos, Chronos["accounts"][4]> {
  public async pda(daemon: PublicKey) {
    return await findPDA(
      [SEED_REVENUE, daemon.toBuffer()],
      this.program.programId
    );
  }
}
