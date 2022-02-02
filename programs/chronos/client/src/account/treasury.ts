import { Gateway, findPDA } from "@chronos-so/utils";
import { Chronos } from "../idl";

const SEED_TREASURY = Buffer.from("treasury");

export class TreasuryGateway extends Gateway<Chronos, Chronos["accounts"][6]> {
  public async pda() {
    return await findPDA([SEED_TREASURY], this.program.programId);
  }
}
