import { Program } from "@project-serum/anchor";
import {
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import { Cronos } from "../idl";
import { Account } from "../account";

export type RevenueCreateArgs = {
  daemon: PublicKey;
  signer: PublicKey;
};

export class RevenueCreate {
  private account: Account;
  private cronos: Program<Cronos>;

  constructor(account: Account, cronos: Program<Cronos>) {
    this.account = account;
    this.cronos = cronos;
  }

  public async revenueCreate({
    daemon,
    signer,
  }: RevenueCreateArgs): Promise<TransactionInstruction> {
    const revenuePDA = await this.account.revenue.pda(daemon);
    return this.cronos.instruction.revenueCreate(revenuePDA.bump, {
      accounts: {
        daemon: daemon,
        revenue: revenuePDA.address,
        signer: signer,
        systemProgram: SystemProgram.programId,
      },
    });
  }
}
