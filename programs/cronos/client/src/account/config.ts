import { Gateway, findPDA } from "@cronos-so/utils";
import { Cronos } from "../idl";

const SEED_CONFIG = Buffer.from("config");

export class ConfigGateway extends Gateway<Cronos, Cronos["accounts"][1]> {
  public async pda() {
    return await findPDA([SEED_CONFIG], this.program.programId);
  }
}
