import { dateToSeconds } from "@cronos-so/utils";
import { BN } from "@project-serum/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { AccountMetaData, ConfigAccountData, InstructionData } from "src";

function nextFrameTimestamp(config: ConfigAccountData): BN {
  const now = new Date();
  const thisFrame = new Date(now.setSeconds(0, 0));
  const nextFrame = new Date(
    thisFrame.getTime() + config.frameInterval.toNumber() * 1000
  );
  return new BN(dateToSeconds(nextFrame));
}

export function buildInstructionData(
  ix: TransactionInstruction
): InstructionData {
  return {
    programId: ix.programId,
    keys: ix.keys as Array<AccountMetaData>,
    data: ix.data,
  };
}

export function buildRemainingAccounts(
  ixData: InstructionData,
  daemon: PublicKey
): Array<AccountMetaData> {
  return (ixData.keys as Array<AccountMetaData>)
    .map((acc) => ({
      pubkey: acc.pubkey,
      isSigner:
        acc.pubkey.toString() === daemon.toString() ? false : acc.isSigner,
      isWritable: acc.isWritable,
    }))
    .concat([
      {
        pubkey: ixData.programId,
        isSigner: false,
        isWritable: false,
      },
    ]);
}
