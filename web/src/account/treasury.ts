import { Gateway, findPDA } from "../utils";
import { Cronos } from "../idl";

const SEED_TREASURY = Buffer.from("treasury");

export class TreasuryGateway extends Gateway<Cronos, Cronos["accounts"][6]> {
  public async pda() {
    return await findPDA([SEED_TREASURY], this.program.programId);
  }
}
