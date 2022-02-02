import { Gateway, findPDA } from "@chronos-so/utils";
import { BN } from "@project-serum/anchor";
import { Chronos } from "../idl";

const SEED_FRAME = Buffer.from("frame");

export class FrameGateway extends Gateway<Chronos, Chronos["accounts"][3]> {
  public async pda(timestamp: BN) {
    return await findPDA(
      [SEED_FRAME, timestamp.toArrayLike(Buffer, "be", 8)],
      this.program.programId
    );
  }
}
