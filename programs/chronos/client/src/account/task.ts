import { Gateway, findPDA } from "@chronos-so/utils";
import { BN } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { Chronos } from "../idl";

const SEED_TASK = Buffer.from("task");

export class TaskGateway extends Gateway<Chronos, Chronos["accounts"][5]> {
  public async pda(daemon: PublicKey, id: BN) {
    return await findPDA(
      [SEED_TASK, daemon.toBuffer(), id.toArrayLike(Buffer, "be", 16)],
      this.program.programId
    );
  }
}
