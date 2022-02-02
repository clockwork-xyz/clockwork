import { Program } from "@project-serum/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { buildInstructionData, buildRemainingAccounts } from "./utils";
import { Chronos } from "../idl";
import { Account } from "../account";

export type DaemonInvokeArgs = {
  daemon: PublicKey;
  instruction: TransactionInstruction;
};

export class DaemonInvoke {
  private account: Account;
  private chronos: Program<Chronos>;

  constructor(account: Account, chronos: Program<Chronos>) {
    this.account = account;
    this.chronos = chronos;
  }

  public async daemonInvoke({
    daemon,
    instruction,
  }: DaemonInvokeArgs): Promise<TransactionInstruction> {
    const daemonData = await this.account.daemon.data(daemon);
    const instructionData = buildInstructionData(instruction);
    return this.chronos.instruction.daemonInvoke(instructionData, {
      accounts: {
        daemon: daemon,
        owner: daemonData.owner,
      },
      remainingAccounts: buildRemainingAccounts(instructionData, daemon),
    });
  }
}
