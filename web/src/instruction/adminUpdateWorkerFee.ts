import { BN, Program } from "@project-serum/anchor";
import { TransactionInstruction } from "@solana/web3.js";
import { Cronos } from "../idl";
import { Account } from "../account";

export type AdminUpdateWorkerFeeArgs = {
  newWorkerFee: BN;
};

export class AdminUpdateWorkerFee {
  private account: Account;
  private cronos: Program<Cronos>;

  constructor(account: Account, cronos: Program<Cronos>) {
    this.account = account;
    this.cronos = cronos;
  }

  public async adminUpdateWorkerFee({
    newWorkerFee,
  }: AdminUpdateWorkerFeeArgs): Promise<TransactionInstruction> {
    const configPDA = await this.account.config.pda();
    const configData = await this.account.config.data(configPDA.address);
    return this.cronos.instruction.adminUpdateWorkerFee(newWorkerFee, {
      accounts: {
        admin: configData.admin,
        config: configPDA.address,
      },
    });
  }
}
