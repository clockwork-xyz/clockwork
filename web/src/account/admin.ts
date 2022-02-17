import { Gateway, findPDA } from "../utils";
import { Cronos } from "../idl";

const SEED_ADMIN = Buffer.from("admin");

export class AdminGateway extends Gateway<Cronos, Cronos["accounts"][1]> {
  public async pda() {
    return await findPDA([SEED_ADMIN], this.program.programId);
  }
}
