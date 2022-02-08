import { Gateway, findPDA } from "@cronos-so/utils";
import { Cronos } from "../idl";

const SEED_AUTHORITY = Buffer.from("authority");

export class AuthorityGateway extends Gateway<Cronos, Cronos["accounts"][0]> {
  public async pda() {
    return await findPDA([SEED_AUTHORITY], this.program.programId);
  }
}
