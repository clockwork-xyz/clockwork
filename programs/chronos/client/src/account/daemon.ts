import { Gateway, findPDA } from "@chronos-so/utils";
import { PublicKey } from "@solana/web3.js";
import { Chronos } from "../idl";

const SEED_DAEMON = Buffer.from("daemon");

export class DaemonGateway extends Gateway<Chronos, Chronos["accounts"][2]> {
  public async pda(owner: PublicKey) {
    return await findPDA(
      [SEED_DAEMON, owner.toBuffer()],
      this.program.programId
    );
  }
}
