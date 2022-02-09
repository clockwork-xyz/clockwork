import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { AccountMetaData, InstructionData } from "src";

export function buildInstructionData(
  ix: TransactionInstruction
): InstructionData {
  return {
    programId: ix.programId,
    accounts: ix.keys as Array<AccountMetaData>,
    data: ix.data,
  };
}

export function buildRemainingAccounts(
  ixData: InstructionData,
  daemon: PublicKey
): Array<AccountMetaData> {
  return (ixData.accounts as Array<AccountMetaData>)
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
