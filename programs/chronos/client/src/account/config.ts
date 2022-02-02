import { Gateway, findPDA } from "@chronos-so/utils";
import { Chronos } from "../idl";

const SEED_CONFIG = Buffer.from("config");

export class ConfigGateway extends Gateway<Chronos, Chronos["accounts"][1]> {
  public async pda() {
    return await findPDA([SEED_CONFIG], this.program.programId);
  }
}
