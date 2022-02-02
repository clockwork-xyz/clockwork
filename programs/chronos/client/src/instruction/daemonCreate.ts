import { Program } from "@project-serum/anchor";
import {
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import { Chronos } from "../idl";
import { Account } from "../account";

export type DaemonCreateArgs = {
  owner: PublicKey;
};

export class DaemonCreate {
  private account: Account;
  private chronos: Program<Chronos>;

  constructor(account: Account, chronos: Program<Chronos>) {
    this.account = account;
    this.chronos = chronos;
  }

  public async daemonCreate({
    owner,
  }: DaemonCreateArgs): Promise<TransactionInstruction> {
    const daemonPDA = await this.account.daemon.pda(owner);
    return this.chronos.instruction.daemonCreate(daemonPDA.bump, {
      accounts: {
        daemon: daemonPDA.address,
        owner: owner,
        systemProgram: SystemProgram.programId,
      },
    });
  }
}
