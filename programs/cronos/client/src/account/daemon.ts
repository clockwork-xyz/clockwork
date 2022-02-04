import { Gateway, findPDA } from "@cronos-so/utils";
import { PublicKey } from "@solana/web3.js";
import { Cronos } from "../idl";

const SEED_DAEMON = Buffer.from("daemon");

export class DaemonGateway extends Gateway<Cronos, Cronos["accounts"][2]> {
  public async pda(owner: PublicKey) {
    return await findPDA(
      [SEED_DAEMON, owner.toBuffer()],
      this.program.programId
    );
  }
}
