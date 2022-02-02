import { Program } from "@project-serum/anchor";
import {
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import { Chronos } from "../idl";
import { Account } from "../account";

export type RevenueCreateArgs = {
  daemon: PublicKey;
  signer: PublicKey;
};

export class RevenueCreate {
  private account: Account;
  private chronos: Program<Chronos>;

  constructor(account: Account, chronos: Program<Chronos>) {
    this.account = account;
    this.chronos = chronos;
  }

  public async revenueCreate({
    daemon,
    signer,
  }: RevenueCreateArgs): Promise<TransactionInstruction> {
    const revenuePDA = await this.account.revenue.pda(daemon);
    return this.chronos.instruction.revenueCreate(revenuePDA.bump, {
      accounts: {
        daemon: daemon,
        revenue: revenuePDA.address,
        signer: signer,
        systemProgram: SystemProgram.programId,
      },
    });
  }
}
