import { Gateway, findPDA } from "@cronos-so/utils";
import { BN } from "@project-serum/anchor";
import { PublicKey } from "@solana/web3.js";
import { Cronos } from "../idl";

const SEED_TASK = Buffer.from("task");

export class TaskGateway extends Gateway<Cronos, Cronos["accounts"][5]> {
  public async pda(daemon: PublicKey, id: BN) {
    return await findPDA(
      [SEED_TASK, daemon.toBuffer(), id.toArrayLike(Buffer, "be", 16)],
      this.program.programId
    );
  }
}
