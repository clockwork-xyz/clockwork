import { Gateway, findPDA } from "@cronos-so/utils";
import { PublicKey } from "@solana/web3.js";
import { Cronos } from "../idl";

const SEED_REVENUE = Buffer.from("revenue");

export class RevenueGateway extends Gateway<Cronos, Cronos["accounts"][3]> {
  public async pda(daemon: PublicKey) {
    return await findPDA(
      [SEED_REVENUE, daemon.toBuffer()],
      this.program.programId
    );
  }
}
