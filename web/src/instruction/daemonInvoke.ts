import { Program } from "@project-serum/anchor";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";
import { buildInstructionData, buildRemainingAccounts } from "./utils";
import { Cronos } from "../idl";
import { Account } from "../account";

export type DaemonInvokeArgs = {
  daemon: PublicKey;
  ix: TransactionInstruction;
};

export class DaemonInvoke {
  private account: Account;
  private cronos: Program<Cronos>;

  constructor(account: Account, cronos: Program<Cronos>) {
    this.account = account;
    this.cronos = cronos;
  }

  public async daemonInvoke({
    daemon,
    ix,
  }: DaemonInvokeArgs): Promise<TransactionInstruction> {
    const daemonData = await this.account.daemon.data(daemon);
    const instructionData = buildInstructionData(ix);
    return this.cronos.instruction.daemonInvoke(instructionData, {
      accounts: {
        daemon: daemon,
        owner: daemonData.owner,
      },
      remainingAccounts: buildRemainingAccounts(instructionData, daemon),
    });
  }
}
