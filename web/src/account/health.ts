import { Gateway, findPDA } from "../utils";
import { PublicKey } from "@solana/web3.js";
import { Cronos } from "../idl";

const SEED_HEALTH = Buffer.from("health");

export class HealthGateway extends Gateway<Cronos, Cronos["accounts"][4]> {
  public async pda() {
    return await findPDA([SEED_HEALTH], this.program.programId);
  }
}
