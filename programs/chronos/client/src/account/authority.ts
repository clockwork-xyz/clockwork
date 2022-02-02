import { Gateway, findPDA } from "@chronos-so/utils";
import { Chronos } from "../idl";

const SEED_AUTHORITY = Buffer.from("authority");

export class AuthorityGateway extends Gateway<Chronos, Chronos["accounts"][0]> {
  public async pda() {
    return await findPDA([SEED_AUTHORITY], this.program.programId);
  }
}
